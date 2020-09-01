use uuid::Uuid;

use crate::models::RSVP;

struct RSVPService {};

impl RSVPService {
    fn get_by_household(&self, household_id: Uuid) -> Result<RSVP> {
        Ok(
            RSVP {

            }
        )
    }

    fn put(&self, id: Uuid, rsvp: RSVP) -> Result<RSVP> {

    }
}
