//! new execution plan types, entirely separate from `execute` crate
mod aggregates;
mod arguments;
mod field;
mod filter;
mod mutation;
mod order_by;
mod query;
mod relationships;
mod remote_joins;
use lang_graphql::ast::common as ast;
use std::sync::Arc;

pub use aggregates::{AggregateFieldSelection, AggregateSelectionSet};
pub use arguments::{Argument, MutationArgument};
pub use field::{Field, NestedArray, NestedField, NestedObject};
pub use filter::ResolvedFilterExpression;
pub use mutation::MutationExecutionPlan;
pub use order_by::{OrderByDirection, OrderByElement, OrderByTarget};
pub use query::{FieldsSelection, PredicateQueryTrees, QueryExecutionPlan, QueryNodeNew};
pub use relationships::{Relationship, RelationshipArgument};
pub use remote_joins::{
    JoinLocations, JoinNode, LocationKind, RemoteJoin, RemoteJoinArgument, SourceFieldAlias,
    TargetField,
};

// these versions of the types are equivalent to the old "Resolved" versions

#[derive(Debug)]
pub struct NDCQueryExecution {
    pub execution_tree: ExecutionTree,
    pub execution_span_attribute: &'static str,
    pub field_span_attribute: String,
    pub process_response_as: ProcessResponseAs,
}

#[derive(Debug)]
pub struct NDCMutationExecution {
    pub execution_node: mutation::MutationExecutionPlan,
    pub join_locations: JoinLocations,
    pub data_connector: Arc<metadata_resolve::DataConnectorLink>,
    pub execution_span_attribute: String,
    pub field_span_attribute: String,
    pub process_response_as: ProcessResponseAs,
    // leaving this out for now as it's GraphQL specific stuff
    // pub selection_set: &'n normalized_ast::SelectionSet<'s, GDS>,
}

#[derive(Debug)]
pub struct ExecutionTree {
    pub query_execution_plan: query::QueryExecutionPlan,
    pub remote_join_executions: remote_joins::JoinLocations,
}

#[derive(Clone, Debug, PartialEq)]
pub enum ProcessResponseAs {
    Object {
        is_nullable: bool,
    },
    Array {
        is_nullable: bool,
    },
    CommandResponse {
        command_name: Arc<metadata_resolve::Qualified<open_dds::commands::CommandName>>,
        type_container: ast::TypeContainer<ast::TypeName>,
        // how to process a command response
        response_config: Option<Arc<metadata_resolve::data_connectors::CommandsResponseConfig>>,
    },
    Aggregates,
}

impl ProcessResponseAs {
    pub fn is_nullable(&self) -> bool {
        match self {
            ProcessResponseAs::Object { is_nullable }
            | ProcessResponseAs::Array { is_nullable } => *is_nullable,
            ProcessResponseAs::CommandResponse { type_container, .. } => type_container.nullable,
            ProcessResponseAs::Aggregates { .. } => false,
        }
    }
}