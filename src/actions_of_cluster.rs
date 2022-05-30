use crate::*;

#[near_bindgen]
impl Contract {

    // #[payable]
    pub fn new_cluster(&mut self, name: String, description: String) -> ClusterId {
        let owner: AccountId = env::predecessor_account_id().into();
        let cluster = Cluster {
            owner_id: owner.clone(),
        };

        let metadata = ClusterMetaData::new(name, description);
        let cluster_id = metadata.clone().id;
        self.cluster.insert(&cluster_id, &cluster);
        self.cluster_metadata.insert(&cluster_id, &metadata);


        // Add cluster to owner
        let mut owner_clusters =
            self.cluster_per_owner
                .get(&owner.clone())
                .unwrap_or(UnorderedSet::new(StorageKey::ClusterPerOwnerInner {
                    id: owner.clone(),
                }));
        owner_clusters.insert(&cluster_id);
        self.cluster_per_owner.insert(&owner,&owner_clusters);
        return cluster_id;
    }
    
}
