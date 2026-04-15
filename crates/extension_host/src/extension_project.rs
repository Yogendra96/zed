use anyhow::{Result, Context as _};
use context_builder::ContextBuilder;
use extension::ProjectDelegate;
use gpui::{AsyncApp, Task, WeakEntity};
use project::Project;
use std::sync::Arc;

pub struct ProjectDelegateAdapter {
    pub project: WeakEntity<Project>,
    pub cx: AsyncApp,
}

impl ProjectDelegate for ProjectDelegateAdapter {
    fn worktree_ids(&self) -> Vec<u64> {
        self.project.update(&mut self.cx.clone(), |project, cx| {
            project.visible_worktrees(cx)
                .map(|w| w.read(cx).id().to_proto())
                .collect()
        }).unwrap_or_default()
    }

    fn get_context(&self) -> Task<Result<String>> {
        let project = self.project.clone();
        let mut cx = self.cx.clone();
        cx.spawn(async move |mut cx| {
            let task = project.update(&mut cx, |project, cx| {
                let project_handle = cx.entity_for_id(project.entity_id())
                    .context("Project handle no longer exists")?;
                let builder = ContextBuilder::new(project_handle);
                builder.build_context(cx)
            })??;
            let context = task.await?;
            Ok(serde_json::to_string(&context)?)
        })
    }
}
