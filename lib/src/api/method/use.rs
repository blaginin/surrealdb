use crate::api::conn::Command;
use crate::api::method::BoxFuture;
use crate::api::Connection;
use crate::api::Result;
use crate::method::OnceLockExt;
use crate::Surreal;
use std::borrow::Cow;
use std::future::IntoFuture;

/// Session info to set up
#[derive(Debug)]
#[must_use = "futures do nothing unless you `.await` or poll them"]
pub struct Use<'r, C: Connection> {
	pub(super) client: Cow<'r, Surreal<C>>,
	pub(super) ns: Option<String>,
	pub(super) db: Option<String>,
	pub(super) session: Option<String>,
}

impl<C> Use<'_, C>
where
	C: Connection,
{
	/// Converts to an owned type which can easily be moved to a different thread
	pub fn into_owned(self) -> Use<'static, C> {
		Use {
			client: Cow::Owned(self.client.into_owned()),
			..self
		}
	}
}

impl<'r, C> Use<'r, C>
where
	C: Connection,
{
	/// Switch to a specific database
	pub fn use_db(self, db: impl Into<String>) -> Self {
		Self {
			db: Some(db.into()),
			..self
		}
	}

	/// Switch to a specific session
	pub fn use_session(self, session: impl Into<String>) -> Self {
		Self {
			session: Some(session.into()),
			..self
		}
	}
}

impl<'r, Client> IntoFuture for Use<'r, Client>
where
	Client: Connection,
{
	type Output = Result<()>;
	type IntoFuture = BoxFuture<'r, Self::Output>;

	fn into_future(self) -> Self::IntoFuture {
		Box::pin(async move {
			let router = self.client.router.extract()?;
			router
				.execute_unit(Command::Use {
					namespace: self.ns,
					database: self.db,
					session_id: self.session,
				})
				.await
		})
	}
}
