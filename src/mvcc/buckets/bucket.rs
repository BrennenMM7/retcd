// Define associated constants for bucket names
const KEY_BUCKET_NAME_BYTES: &[u8] = b"key";
const META_BUCKET_NAME_BYTES: &[u8] = b"meta";
const LEASE_BUCKET_NAME_BYTES: &[u8] = b"lease";
const ALARM_BUCKET_NAME_BYTES: &[u8] = b"alarm";
const CLUSTER_BUCKET_NAME_BYTES: &[u8] = b"cluster";
const MEMBERS_BUCKET_NAME_BYTES: &[u8] = b"members";
const MEMBERS_REMOVED_BUCKET_NAME_BYTES: &[u8] = b"members_removed";
const AUTH_BUCKET_NAME_BYTES: &[u8] = b"auth";
const AUTH_USERS_BUCKET_NAME_BYTES: &[u8] = b"authUsers";
const AUTH_ROLES_BUCKET_NAME_BYTES: &[u8] = b"authRoles";

pub const KEY: Bucket = Bucket::new(1, KEY_BUCKET_NAME_BYTES, true);
const META: Bucket = Bucket::new(2, META_BUCKET_NAME_BYTES, false);
const LEASE: Bucket = Bucket::new(3, LEASE_BUCKET_NAME_BYTES, false);
const ALARM: Bucket = Bucket::new(4, ALARM_BUCKET_NAME_BYTES, false);
const CLUSTER: Bucket = Bucket::new(5, CLUSTER_BUCKET_NAME_BYTES, false);
const MEMBERS: Bucket = Bucket::new(10, MEMBERS_BUCKET_NAME_BYTES, false);
const MEMBERS_REMOVED: Bucket = Bucket::new(11, MEMBERS_REMOVED_BUCKET_NAME_BYTES, false);
const AUTH: Bucket = Bucket::new(20, AUTH_BUCKET_NAME_BYTES, false);
const AUTH_USERS: Bucket = Bucket::new(21, AUTH_USERS_BUCKET_NAME_BYTES, false);
const AUTH_ROLES: Bucket = Bucket::new(22, AUTH_ROLES_BUCKET_NAME_BYTES, false);

type BucketID = i32;

pub struct Bucket {
    pub id: BucketID,
    pub name: &'static [u8],
    pub safe_range_bucket: bool,
}

impl Bucket {
    const fn new(id: BucketID, name: &'static [u8], safe_range_bucket: bool) -> Self {
        Bucket {
            id,
            name,
            safe_range_bucket,
        }
    }

    pub fn id(&self) -> BucketID {
        self.id
    }

    pub fn name(&self) -> &[u8] {
        &self.name
    }
    
    pub fn string(&self) -> String {
        String::from_utf8_lossy(self.name).to_string()
    }

    pub fn is_safe_range_bucket(&self) -> bool {
        self.safe_range_bucket
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_bucket_id() {
        assert_eq!(KEY.id(), 1);
        assert_eq!(META.id(), 2);
        assert_eq!(LEASE.id(), 3);
        assert_eq!(ALARM.id(), 4);
        assert_eq!(CLUSTER.id(), 5);
        assert_eq!(MEMBERS.id(), 10);
        assert_eq!(MEMBERS_REMOVED.id(), 11);
        assert_eq!(AUTH.id(), 20);
        assert_eq!(AUTH_USERS.id(), 21);
        assert_eq!(AUTH_ROLES.id(), 22);
    }

    #[test]
    fn test_bucket_name() {
        assert_eq!(KEY.name(), KEY_BUCKET_NAME_BYTES);
        assert_eq!(META.name(), META_BUCKET_NAME_BYTES);
        assert_eq!(LEASE.name(), LEASE_BUCKET_NAME_BYTES);
        assert_eq!(ALARM.name(), ALARM_BUCKET_NAME_BYTES);
        assert_eq!(CLUSTER.name(), CLUSTER_BUCKET_NAME_BYTES);
        assert_eq!(MEMBERS.name(), MEMBERS_BUCKET_NAME_BYTES);
        assert_eq!(MEMBERS_REMOVED.name(), MEMBERS_REMOVED_BUCKET_NAME_BYTES);
        assert_eq!(AUTH.name(), AUTH_BUCKET_NAME_BYTES);
        assert_eq!(AUTH_USERS.name(), AUTH_USERS_BUCKET_NAME_BYTES);
        assert_eq!(AUTH_ROLES.name(), AUTH_ROLES_BUCKET_NAME_BYTES);
    }

    #[test]
    fn test_bucket_string() {
        assert_eq!(KEY.string(), String::from_utf8_lossy(KEY_BUCKET_NAME_BYTES).to_string());
        assert_eq!(META.string(), String::from_utf8_lossy(META_BUCKET_NAME_BYTES).to_string());
        assert_eq!(LEASE.string(), String::from_utf8_lossy(LEASE_BUCKET_NAME_BYTES).to_string());
        assert_eq!(ALARM.string(), String::from_utf8_lossy(ALARM_BUCKET_NAME_BYTES).to_string());
        assert_eq!(CLUSTER.string(), String::from_utf8_lossy(CLUSTER_BUCKET_NAME_BYTES).to_string());
        assert_eq!(MEMBERS.string(), String::from_utf8_lossy(MEMBERS_BUCKET_NAME_BYTES).to_string());
        assert_eq!(MEMBERS_REMOVED.string(), String::from_utf8_lossy(MEMBERS_REMOVED_BUCKET_NAME_BYTES).to_string());
        assert_eq!(AUTH.string(), String::from_utf8_lossy(AUTH_BUCKET_NAME_BYTES).to_string());
        assert_eq!(AUTH_USERS.string(), String::from_utf8_lossy(AUTH_USERS_BUCKET_NAME_BYTES).to_string());
        assert_eq!(AUTH_ROLES.string(), String::from_utf8_lossy(AUTH_ROLES_BUCKET_NAME_BYTES).to_string());
    }

    #[test]
    fn test_bucket_is_safe_range_bucket() {
        assert_eq!(KEY.is_safe_range_bucket(), true);
        assert_eq!(META.is_safe_range_bucket(), false);
        assert_eq!(LEASE.is_safe_range_bucket(), false);
        assert_eq!(ALARM.is_safe_range_bucket(), false);
        assert_eq!(CLUSTER.is_safe_range_bucket(), false);
        assert_eq!(MEMBERS.is_safe_range_bucket(), false);
        assert_eq!(MEMBERS_REMOVED.is_safe_range_bucket(), false);
        assert_eq!(AUTH.is_safe_range_bucket(), false);
        assert_eq!(AUTH_USERS.is_safe_range_bucket(), false);
        assert_eq!(AUTH_ROLES.is_safe_range_bucket(), false);
    }
}
