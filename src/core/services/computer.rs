use async_trait::async_trait;

use crate::{
    core::models::{List, Task},
    core::task_service::TasksProvider,
    Error,
};

use engine::ComputerStorageEngine;

mod engine;

#[derive(Debug, Clone)]
pub struct ComputerStorage {
    engine: ComputerStorageEngine,
}

impl ComputerStorage {
    pub(crate) fn new(application_id: &str) -> Option<Self> {
        ComputerStorageEngine::new(application_id).map(|engine| Self { engine })
    }
}

#[async_trait]
impl TasksProvider for ComputerStorage {
    async fn get_task(&mut self, list_id: String, task_id: String) -> Result<Task, Error> {
        self.engine.get_task(&list_id, &task_id)
    }

    async fn get_tasks_from_list(&mut self, parent_list: String) -> Result<Vec<Task>, Error> {
        self.engine.tasks(&parent_list)
    }

    async fn create_task(&mut self, task: Task) -> Result<Task, Error> {
        self.engine.create_task(task)
    }

    async fn update_task(&mut self, task: Task) -> Result<(), Error> {
        self.engine.update_task(task)
    }

    async fn delete_task(&mut self, list_id: String, task_id: String) -> Result<(), Error> {
        self.engine.delete_task(&list_id, &task_id)
    }

    async fn get_lists(&mut self) -> Result<Vec<List>, Error> {
        self.engine.lists()
    }

    async fn get_list(&mut self, id: String) -> Result<List, Error> {
        self.engine.get_list(&id)
    }

    async fn create_list(&mut self, list: List) -> Result<List, Error> {
        self.engine.create_list(list)
    }

    async fn update_list(&mut self, list: List) -> Result<(), Error> {
        self.engine.update_list(list)
    }

    async fn delete_list(&mut self, id: String) -> Result<(), Error> {
        self.engine.delete_list(&id)
    }
}
