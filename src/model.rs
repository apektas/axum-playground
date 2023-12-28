//! Simplistic Model Layer (with mock-store layer)

use crate::{Error, Result};
use serde::{Deserialize, Serialize};
use std::sync::{Arc, Mutex};
use crate::ctx::Ctx;


// region - Ticket Types

#[derive(Clone, Debug, Serialize)]
pub struct Ticket{
    pub id: u64,
    pub cid: u64, // creator user
    pub title: String,
}


#[derive(Deserialize)]
pub struct TicketForCreate{
    pub title: String,
}

// endregion



// region - Model Controller

#[derive(Clone)]
// just clone the arc not the vector
pub struct ModelController {
    tickets_store: Arc<Mutex<Vec<Option<Ticket>>>>,
}

// constructor
impl ModelController {
    pub async fn new() -> Result<Self> {
        Ok( Self {
            tickets_store: Arc::default(),
        })
    }
}

impl ModelController {
    pub async fn create_ticket(&self, ctx: Ctx, ticket_to_create: TicketForCreate) -> Result<Ticket>{
        let mut store = self.tickets_store.lock().unwrap();

        let id = store.len() as u64;
        let ticket = Ticket {
            id,
            cid: ctx.user_id(),
            title: ticket_to_create.title,
        };

        store.push(Some(ticket.clone()));

        Ok(ticket)

    }

    pub async fn list_tickets(&self,  _ctx: Ctx) -> Result<Vec<Ticket>> {

        let store = self.tickets_store.lock().unwrap();

        let ticket = store.iter().filter_map(|t| t.clone()).collect::<Vec<Ticket>>();
        Ok(ticket)

    }

    pub async fn delete_ticket(&self,  _ctx: Ctx,  id: u64) -> Result<Ticket> {
        let mut store = self.tickets_store.lock().unwrap();
        let ticket = store.get_mut(id as usize).and_then(|t| t.take());

        ticket.ok_or(Error::TicketDeleteFailIdNotFound {id})

    }

}


// endregion