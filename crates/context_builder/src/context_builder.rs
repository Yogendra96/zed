use anyhow::{Result, anyhow};
use gpui::{AppContext, Entity, Model};
use project::Project;
use serde::{Deserialize, Serialize};
use std::path::Path;
use workspace::Workspace;
use smol::process::Command;

#[derive(Serialize, Deserialize, Debug)]
pub struct ContextEnvelope {
    pub open_files: Vec<FileContext>,
    pub git_diffs: Vec<GitDiffContext>,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct FileContext {
    pub path: String,
    pub content: String,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct GitDiffContext {
    pub worktree_root: String,
    pub diff: String,
}

pub struct ContextBuilder {
    project: Entity<Project>,
}

impl ContextBuilder {
    pub fn new(project: Entity<Project>) -> Self {
        Self { project }
    }

    pub async fn build_context(&self, cx: &mut AppContext) -> Result<ContextEnvelope> {
        let mut open_files = Vec::new();
        let mut git_diffs = Vec::new();

        // 1. Collect open buffers
        let buffers = self.project.update(cx, |project, cx| {
            project.opened_buffers(cx)
        });

        for buffer in buffers {
            buffer.update(cx, |buffer, _| {
                if let Some(file) = buffer.file() {
                    open_files.push(FileContext {
                        path: file.path().to_string_lossy().to_string(),
                        content: buffer.text().to_string(),
                    });
                }
            });
        }

        // 2. Collect Git Diffs from all visible worktrees
        let worktree_roots = self.project.update(cx, |project, cx| {
            project.visible_worktrees(cx)
                .map(|wt| wt.read(cx).abs_path().to_path_buf())
                .collect::<Vec<_>>()
        });

        for root in worktree_roots {
            if let Ok(diff) = self.get_git_diff(&root).await {
                if !diff.trim().is_empty() {
                    git_diffs.push(GitDiffContext {
                        worktree_root: root.to_string_lossy().to_string(),
                        diff,
                    });
                }
            }
        }

        Ok(ContextEnvelope {
            open_files,
            git_diffs,
        })
    }

    async fn get_git_diff(&self, root: &Path) -> Result<String> {
        // Get both staged and unstaged changes
        let output = Command::new("git")
            .arg("diff")
            .arg("HEAD")
            .current_dir(root)
            .output()
            .await?;

        if output.status.success() {
            Ok(String::from_utf8_lossy(&output.stdout).to_string())
        } else {
            Err(anyhow!("git diff failed in {:?}", root))
        }
    }
}
