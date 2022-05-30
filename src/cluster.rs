use crate::*;

#[derive(BorshDeserialize, BorshSerialize,Serialize, Deserialize)]
#[serde(crate = "near_sdk::serde")]
pub struct Cluster {
    pub owner_id: AccountId,
}

#[derive(BorshDeserialize, BorshSerialize, Serialize, Deserialize, Debug, Clone)]
#[serde(crate = "near_sdk::serde")]
pub struct ClusterMetaData {
    pub id: String,
    pub name: String,
    pub description: String,
    pub api_key: String,
    pub data: String,
}

impl ClusterMetaData {
    pub fn new(name: String, description: String) -> Self {
        Self {
            id: gen_cluster_id(),
            name: name,
            description: description,
            api_key: generate_api_key(),
            data: String::from(""),
        }
    }
}

#[near_bindgen]
impl Contract {
    pub fn get_clusters(&mut self) -> Vec<ClusterMetaData> {
        let owner = env::predecessor_account_id();

        let clusters_for_owner_set = self.cluster_per_owner.get(&owner);
        let clusters = if let Some(clusters_for_owner_set) = clusters_for_owner_set {
            clusters_for_owner_set
        } else {
            return vec![];
        };
        return clusters
            .iter()
            .skip(0 as usize)
            .take(50 as usize)
            .map(|cluster_id| self.cluster_metadata.get(&cluster_id).unwrap())
            .collect();
    }

    pub fn get_cluster(&mut self, id:String) -> Cluster{
        let cluster = self.cluster.get(&id);  

        assert!(
            cluster.is_some(),
            "Cluster is not exist!"
        );

        assert!(
            cluster.as_ref().unwrap().owner_id.to_string() == env::signer_account_id(),
            "This cluster is not belong to you"
        );

        return cluster.unwrap();
        
    }
    pub fn get_cluster_data(&mut self, id:String) -> ClusterMetaData{
        let cluster = self.cluster.get(&id);  

        assert!(
            cluster.is_some(),
            "Cluster is not exist!"
        );

        assert!(
            cluster.as_ref().unwrap().owner_id.to_string() == env::signer_account_id(),
            "This cluster is not belong to you"
        );

        return self.cluster_metadata.get(&id).unwrap();
        
    }
}
