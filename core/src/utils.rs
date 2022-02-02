use serde::Deserialize;
use serde::Serialize;

// TODO: fix naming on ALL of these

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct ClassListResponse {
    pub rxclass_min_concept_list: RxclassMinConceptList,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct RxclassMinConceptList {
    pub rxclass_min_concept: Vec<RxclassMinConcept>,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct RxclassMinConcept {
    pub class_id: String,
    pub class_name: String,
    pub class_type: String,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct ClassMembersResponse {
    pub drug_member_group: DrugMemberGroup,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct DrugMemberGroup {
    pub drug_member: Vec<DrugMember>,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct DrugMember {
    pub min_concept: MinConcept,
    pub node_attr: Vec<NodeAttr>,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct MinConcept {
    pub rxcui: String,
    pub name: String,
    pub tty: String,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct NodeAttr {
    pub attr_name: String,
    pub attr_value: String,
}
