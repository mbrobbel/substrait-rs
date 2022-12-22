use crate::proto::{
    rel::RelType, rel_common::EmitKind, FilterRel, NamedStruct, ReadRel, Rel, RelCommon, *,
};

pub trait Relation {
    fn common(&self) -> &RelCommon;
    fn schema(&self) -> NamedStruct;
    // fn schema(&self) -> Schema; introduce new type for this
}

impl NamedStruct {
    fn apply_emit(&self, emit_kind: &EmitKind) -> NamedStruct {
        self.clone()
    }
    fn concat(&self, other: NamedStruct) -> NamedStruct {
        // detect collisions and other stuff
        // let new = self.clone();
        // new.names.extend(other.names.into_iter());
        // new.struct
        // todo really
        self.clone()
    }
}

#[derive(Debug)]
struct Schema<'a> {
    fields: Vec<Field<'a>>,
}

#[derive(Debug)]
struct Field<'a> {
    name: &'a str,
    r#type: Type<'a>,
}

#[derive(Debug)]
enum Type<'a> {
    Bool,
    Int,
    String,
    Struct(Schema<'a>),
    Decimal,
    Date,
}

impl<'a> From<&'a NamedStruct> for Schema<'a> {
    fn from(value: &'a NamedStruct) -> Self {
        let mut names = value.names.iter();
        let r#struct = value.r#struct.as_ref().unwrap();
        let fields = r#struct
            .types
            .iter()
            .map(|ty| -> Field {
                Field {
                    name: names.next().unwrap().as_ref(),
                    r#type: match ty.kind.as_ref().unwrap() {
                        r#type::Kind::Bool(_) => Type::Bool,
                        r#type::Kind::I8(_) => todo!(),
                        r#type::Kind::I16(_) => todo!(),
                        r#type::Kind::I32(_) => Type::Int,
                        r#type::Kind::I64(_) => Type::Int,
                        r#type::Kind::Fp32(_) => todo!(),
                        r#type::Kind::Fp64(_) => todo!(),
                        r#type::Kind::String(_) => todo!(),
                        r#type::Kind::Binary(_) => todo!(),
                        r#type::Kind::Timestamp(_) => todo!(),
                        r#type::Kind::Date(_) => Type::Date,
                        r#type::Kind::Time(_) => todo!(),
                        r#type::Kind::IntervalYear(_) => todo!(),
                        r#type::Kind::IntervalDay(_) => todo!(),
                        r#type::Kind::TimestampTz(_) => todo!(),
                        r#type::Kind::Uuid(_) => todo!(),
                        r#type::Kind::FixedChar(_) => Type::String,
                        r#type::Kind::Varchar(_) => Type::String,
                        r#type::Kind::FixedBinary(_) => todo!(),
                        r#type::Kind::Decimal(_) => Type::Decimal,
                        r#type::Kind::Struct(_) => todo!(),
                        r#type::Kind::List(_) => todo!(),
                        r#type::Kind::Map(_) => todo!(),
                        r#type::Kind::UserDefined(_) => todo!(),
                        r#type::Kind::UserDefinedTypeReference(_) => todo!(),
                    },
                }
            })
            .collect();
        Schema { fields }
    }
}

impl Relation for Rel {
    fn common(&self) -> &RelCommon {
        match self.rel_type.as_ref() {
            Some(rel) => match rel {
                RelType::Read(read_rel) => read_rel.common(),
                RelType::Filter(filter_rel) => filter_rel.common(),
                RelType::Fetch(fetch_rel) => fetch_rel.common(),
                RelType::Aggregate(aggregate_rel) => aggregate_rel.common(),
                RelType::Sort(sort_rel) => sort_rel.common(),
                RelType::Join(join_rel) => join_rel.common(),
                RelType::Project(project_rel) => project_rel.common(),
                // RelType::Set(_) => todo!(),
                // RelType::ExtensionSingle(_) => todo!(),
                // RelType::ExtensionMulti(_) => todo!(),
                // RelType::ExtensionLeaf(_) => todo!(),
                // RelType::Cross(_) => todo!(),
                _ => todo!(),
            },
            None => todo!(),
        }
    }

    fn schema(&self) -> NamedStruct {
        match self.rel_type.as_ref() {
            Some(rel) => match rel {
                RelType::Read(read_rel) => read_rel.schema(),
                RelType::Filter(filter_rel) => filter_rel.schema(),
                RelType::Fetch(fetch_rel) => fetch_rel.schema(),
                RelType::Aggregate(aggregate_rel) => aggregate_rel.schema(),
                RelType::Sort(sort_rel) => sort_rel.schema(),
                RelType::Join(join_rel) => join_rel.schema(),
                RelType::Project(project_rel) => project_rel.schema(),
                // RelType::Set(_) => todo!(),
                // RelType::ExtensionSingle(_) => todo!(),
                // RelType::ExtensionMulti(_) => todo!(),
                // RelType::ExtensionLeaf(_) => todo!(),
                // RelType::Cross(_) => todo!(),
                _ => todo!(),
            },
            None => todo!(),
        }
    }
}

impl Relation for ReadRel {
    fn common(&self) -> &RelCommon {
        self.common.as_ref().unwrap()
    }

    fn schema(&self) -> NamedStruct {
        // This field is required so if missing this is a plan error.
        self.base_schema.clone().unwrap()
    }
}

impl Relation for ProjectRel {
    fn common(&self) -> &RelCommon {
        self.common.as_ref().unwrap()
    }

    fn schema(&self) -> NamedStruct {
        self.input
            .as_ref()
            .unwrap()
            .schema()
            .apply_emit(self.common().emit_kind.as_ref().unwrap())
        // todo add the project expressions
    }
}

impl Relation for JoinRel {
    fn common(&self) -> &RelCommon {
        self.common.as_ref().unwrap()
    }

    fn schema(&self) -> NamedStruct {
        // schema for join depends on join type
        // for now just concat all fields
        self.left
            .as_ref()
            .unwrap()
            .schema()
            .concat(self.right.as_ref().unwrap().schema())
    }
}

impl Relation for AggregateRel {
    fn common(&self) -> &RelCommon {
        self.common.as_ref().unwrap()
    }

    fn schema(&self) -> NamedStruct {
        // todo
        self.input.as_ref().unwrap().schema()
    }
}

impl Relation for SortRel {
    fn common(&self) -> &RelCommon {
        self.common.as_ref().unwrap()
    }

    fn schema(&self) -> NamedStruct {
        self.input
            .as_ref()
            .unwrap()
            .schema()
            .apply_emit(self.common().emit_kind.as_ref().unwrap())
    }
}

impl Relation for FetchRel {
    fn common(&self) -> &RelCommon {
        self.common.as_ref().unwrap()
    }

    fn schema(&self) -> NamedStruct {
        self.input
            .as_ref()
            .unwrap()
            .schema()
            .apply_emit(self.common().emit_kind.as_ref().unwrap())
    }
}

impl Relation for FilterRel {
    fn common(&self) -> &RelCommon {
        self.common.as_ref().unwrap()
    }

    fn schema(&self) -> NamedStruct {
        // input of source
        self.input
            .as_ref()
            .unwrap()
            .schema()
            .apply_emit(self.common().emit_kind.as_ref().unwrap())
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    const TPCH01: [u8; 1468] = [
        10u8, 28, 8, 1, 18, 24, 47, 102, 117, 110, 99, 116, 105, 111, 110, 115, 95, 100, 97, 116,
        101, 116, 105, 109, 101, 46, 121, 97, 109, 108, 10, 38, 8, 2, 18, 34, 47, 102, 117, 110,
        99, 116, 105, 111, 110, 115, 95, 97, 114, 105, 116, 104, 109, 101, 116, 105, 99, 95, 100,
        101, 99, 105, 109, 97, 108, 46, 121, 97, 109, 108, 10, 37, 8, 3, 18, 33, 47, 102, 117, 110,
        99, 116, 105, 111, 110, 115, 95, 97, 103, 103, 114, 101, 103, 97, 116, 101, 95, 103, 101,
        110, 101, 114, 105, 99, 46, 121, 97, 109, 108, 18, 21, 26, 19, 8, 1, 16, 1, 26, 13, 108,
        116, 101, 58, 100, 97, 116, 101, 95, 100, 97, 116, 101, 18, 25, 26, 23, 8, 1, 16, 2, 26,
        17, 115, 117, 98, 116, 114, 97, 99, 116, 58, 100, 97, 116, 101, 95, 100, 97, 121, 18, 28,
        26, 26, 8, 2, 16, 3, 26, 20, 109, 117, 108, 116, 105, 112, 108, 121, 58, 111, 112, 116, 95,
        100, 101, 99, 95, 100, 101, 99, 18, 28, 26, 26, 8, 2, 16, 4, 26, 20, 115, 117, 98, 116,
        114, 97, 99, 116, 58, 111, 112, 116, 95, 100, 101, 99, 95, 100, 101, 99, 18, 23, 26, 21, 8,
        2, 16, 5, 26, 15, 97, 100, 100, 58, 111, 112, 116, 95, 100, 101, 99, 95, 100, 101, 99, 18,
        19, 26, 17, 8, 2, 16, 6, 26, 11, 115, 117, 109, 58, 111, 112, 116, 95, 100, 101, 99, 18,
        19, 26, 17, 8, 2, 16, 7, 26, 11, 97, 118, 103, 58, 111, 112, 116, 95, 100, 101, 99, 18, 17,
        26, 15, 8, 3, 16, 8, 26, 9, 99, 111, 117, 110, 116, 58, 111, 112, 116, 26, 246, 8, 18, 243,
        8, 10, 244, 7, 42, 241, 7, 10, 2, 10, 0, 18, 204, 7, 34, 201, 7, 10, 14, 18, 12, 10, 10, 0,
        1, 2, 3, 4, 5, 6, 7, 8, 9, 18, 197, 5, 58, 194, 5, 10, 11, 18, 9, 10, 7, 16, 17, 18, 19,
        20, 21, 22, 18, 165, 3, 18, 162, 3, 10, 2, 10, 0, 18, 229, 2, 10, 226, 2, 10, 2, 10, 0, 18,
        207, 2, 10, 10, 76, 95, 79, 82, 68, 69, 82, 75, 69, 89, 10, 9, 76, 95, 80, 65, 82, 84, 75,
        69, 89, 10, 9, 76, 95, 83, 85, 80, 80, 75, 69, 89, 10, 12, 76, 95, 76, 73, 78, 69, 78, 85,
        77, 66, 69, 82, 10, 10, 76, 95, 81, 85, 65, 78, 84, 73, 84, 89, 10, 15, 76, 95, 69, 88, 84,
        69, 78, 68, 69, 68, 80, 82, 73, 67, 69, 10, 10, 76, 95, 68, 73, 83, 67, 79, 85, 78, 84, 10,
        5, 76, 95, 84, 65, 88, 10, 12, 76, 95, 82, 69, 84, 85, 82, 78, 70, 76, 65, 71, 10, 12, 76,
        95, 76, 73, 78, 69, 83, 84, 65, 84, 85, 83, 10, 10, 76, 95, 83, 72, 73, 80, 68, 65, 84, 69,
        10, 12, 76, 95, 67, 79, 77, 77, 73, 84, 68, 65, 84, 69, 10, 13, 76, 95, 82, 69, 67, 69, 73,
        80, 84, 68, 65, 84, 69, 10, 14, 76, 95, 83, 72, 73, 80, 73, 78, 83, 84, 82, 85, 67, 84, 10,
        10, 76, 95, 83, 72, 73, 80, 77, 79, 68, 69, 10, 9, 76, 95, 67, 79, 77, 77, 69, 78, 84, 18,
        128, 1, 10, 4, 58, 2, 16, 2, 10, 4, 58, 2, 16, 2, 10, 4, 58, 2, 16, 2, 10, 4, 42, 2, 16, 1,
        10, 7, 194, 1, 4, 16, 19, 32, 1, 10, 7, 194, 1, 4, 16, 19, 32, 1, 10, 7, 194, 1, 4, 16, 19,
        32, 1, 10, 7, 194, 1, 4, 16, 19, 32, 1, 10, 7, 170, 1, 4, 8, 1, 24, 1, 10, 7, 170, 1, 4, 8,
        1, 24, 1, 10, 5, 130, 1, 2, 16, 1, 10, 5, 130, 1, 2, 16, 1, 10, 5, 130, 1, 2, 16, 1, 10, 7,
        170, 1, 4, 8, 25, 24, 1, 10, 7, 170, 1, 4, 8, 10, 24, 1, 10, 7, 178, 1, 4, 8, 44, 24, 1,
        24, 2, 58, 10, 10, 8, 76, 73, 78, 69, 73, 84, 69, 77, 26, 52, 26, 50, 8, 1, 18, 10, 18, 8,
        10, 4, 18, 2, 8, 10, 34, 0, 18, 28, 26, 26, 8, 2, 18, 6, 10, 4, 128, 1, 193, 82, 18, 7, 10,
        5, 162, 1, 2, 8, 120, 26, 5, 130, 1, 2, 16, 2, 26, 4, 10, 2, 16, 1, 26, 10, 18, 8, 10, 4,
        18, 2, 8, 8, 34, 0, 26, 10, 18, 8, 10, 4, 18, 2, 8, 9, 34, 0, 26, 10, 18, 8, 10, 4, 18, 2,
        8, 4, 34, 0, 26, 10, 18, 8, 10, 4, 18, 2, 8, 5, 34, 0, 26, 71, 26, 69, 8, 3, 18, 10, 18, 8,
        10, 4, 18, 2, 8, 5, 34, 0, 18, 44, 26, 42, 8, 4, 18, 17, 90, 15, 10, 7, 194, 1, 4, 16, 19,
        32, 1, 18, 4, 10, 2, 40, 1, 18, 10, 18, 8, 10, 4, 18, 2, 8, 6, 34, 0, 26, 7, 194, 1, 4, 16,
        19, 32, 1, 26, 7, 194, 1, 4, 16, 19, 32, 1, 26, 133, 1, 26, 130, 1, 8, 3, 18, 71, 26, 69,
        8, 3, 18, 10, 18, 8, 10, 4, 18, 2, 8, 5, 34, 0, 18, 44, 26, 42, 8, 4, 18, 17, 90, 15, 10,
        7, 194, 1, 4, 16, 19, 32, 1, 18, 4, 10, 2, 40, 1, 18, 10, 18, 8, 10, 4, 18, 2, 8, 6, 34, 0,
        26, 7, 194, 1, 4, 16, 19, 32, 1, 26, 7, 194, 1, 4, 16, 19, 32, 1, 18, 44, 26, 42, 8, 5, 18,
        17, 90, 15, 10, 7, 194, 1, 4, 16, 19, 32, 1, 18, 4, 10, 2, 40, 1, 18, 10, 18, 8, 10, 4, 18,
        2, 8, 7, 34, 0, 26, 7, 194, 1, 4, 16, 19, 32, 1, 26, 7, 194, 1, 4, 16, 19, 32, 1, 26, 10,
        18, 8, 10, 4, 18, 2, 8, 6, 34, 0, 26, 22, 10, 8, 18, 6, 10, 2, 18, 0, 34, 0, 10, 10, 18, 8,
        10, 4, 18, 2, 8, 1, 34, 0, 34, 27, 10, 25, 8, 6, 18, 10, 18, 8, 10, 4, 18, 2, 8, 2, 34, 0,
        32, 3, 42, 7, 194, 1, 4, 16, 19, 32, 1, 34, 27, 10, 25, 8, 6, 18, 10, 18, 8, 10, 4, 18, 2,
        8, 3, 34, 0, 32, 3, 42, 7, 194, 1, 4, 16, 19, 32, 1, 34, 27, 10, 25, 8, 6, 18, 10, 18, 8,
        10, 4, 18, 2, 8, 4, 34, 0, 32, 3, 42, 7, 194, 1, 4, 16, 19, 32, 1, 34, 27, 10, 25, 8, 6,
        18, 10, 18, 8, 10, 4, 18, 2, 8, 5, 34, 0, 32, 3, 42, 7, 194, 1, 4, 16, 19, 32, 1, 34, 27,
        10, 25, 8, 7, 18, 10, 18, 8, 10, 4, 18, 2, 8, 2, 34, 0, 32, 3, 42, 7, 194, 1, 4, 16, 19,
        32, 1, 34, 27, 10, 25, 8, 7, 18, 10, 18, 8, 10, 4, 18, 2, 8, 3, 34, 0, 32, 3, 42, 7, 194,
        1, 4, 16, 19, 32, 1, 34, 27, 10, 25, 8, 7, 18, 10, 18, 8, 10, 4, 18, 2, 8, 6, 34, 0, 32, 3,
        42, 7, 194, 1, 4, 16, 19, 32, 1, 34, 12, 10, 10, 8, 8, 32, 3, 42, 4, 58, 2, 16, 2, 26, 12,
        10, 8, 18, 6, 10, 2, 18, 0, 34, 0, 16, 2, 26, 14, 10, 10, 18, 8, 10, 4, 18, 2, 8, 1, 34, 0,
        16, 2, 18, 12, 76, 95, 82, 69, 84, 85, 82, 78, 70, 76, 65, 71, 18, 12, 76, 95, 76, 73, 78,
        69, 83, 84, 65, 84, 85, 83, 18, 7, 83, 85, 77, 95, 81, 84, 89, 18, 14, 83, 85, 77, 95, 66,
        65, 83, 69, 95, 80, 82, 73, 67, 69, 18, 14, 83, 85, 77, 95, 68, 73, 83, 67, 95, 80, 82, 73,
        67, 69, 18, 10, 83, 85, 77, 95, 67, 72, 65, 82, 71, 69, 18, 7, 65, 86, 71, 95, 81, 84, 89,
        18, 9, 65, 86, 71, 95, 80, 82, 73, 67, 69, 18, 8, 65, 86, 71, 95, 68, 73, 83, 67, 18, 11,
        67, 79, 85, 78, 84, 95, 79, 82, 68, 69, 82, 50, 16, 42, 14, 118, 97, 108, 105, 100, 97,
        116, 111, 114, 45, 116, 101, 115, 116,
    ];

    #[test]
    fn check() {
        use prost::Message;
        let plan = crate::proto::Plan::decode(TPCH01.as_slice()).unwrap();
        for rel in plan.relations {
            if let Some(rel_type) = rel.rel_type {
                match rel_type {
                    crate::proto::plan_rel::RelType::Rel(rel)
                    | crate::proto::plan_rel::RelType::Root(crate::proto::RelRoot {
                        input: Some(rel),
                        ..
                    }) => {
                        dbg!(rel.schema());
                        dbg!(Schema::from(&rel.schema()));
                    }
                    _ => {}
                }
            }
        }
    }
}
