//!
//! This crate provides data-types defined by the [VDA5050](https://github.com/VDA5050/VDA5050)
//! standard. VDA5050 is an open standard for communication between AGV fleets and a central master control.
//!
//! # Crate Features
//!
//! Enable or disable features according to your needs and in order to optimize for compile time and space.
//!
//! | Feature   | Default  | Description                                                                                                            |
//! | --------- |:--------:| ---------------------------------------------------------------------------------------------------------------------- |
//! | fmt       | &#x2714; | When enabled, certain types will provide an implementation for [`core::fmt::Debug`] and [`core::fmt::Display`] traits. |
//! | serde     | &#x2717; | When enabled, certain types will provide an implementation for [`serde::Serialize`] and [`serde::Deserialize`] traits. |
//! | v2_0      | &#x2717; | When enabled, VDA5050 version 2 types are available.                                                                   |
//!
//! <sup>&#x2714; enabled, &#x2717; disabled</sup>
//!
#![cfg_attr(not(test), no_std)]
#![cfg_attr(docsrs, feature(doc_cfg))]

#[cfg(test)]
extern crate std;

extern crate alloc;

mod action;
mod common;
mod connection;
mod factsheet;
mod instant_actions;
mod order;
mod state;
mod visualization;

#[cfg(any(feature = "v2_0", doc))]
#[cfg_attr(docsrs, doc(cfg(feature = "v2_0")))]
pub mod v2_0 {

    pub mod common {
        pub use crate::action::Action;
        pub use crate::action::ActionParameter;
        pub use crate::action::BlockingType;

        pub use crate::common::AgvPosition;
        pub use crate::common::BoundingBoxReference;
        pub use crate::common::ControlPoint;
        pub use crate::common::HeaderId;
        pub use crate::common::LoadDimensions;
        pub use crate::common::NodePosition;
        pub use crate::common::Timestamp;
        pub use crate::common::Trajectory;
        pub use crate::common::Velocity;
    }

    pub mod connection {
        pub use crate::connection::Connection;
        pub use crate::connection::ConnectionState;
    }

    pub mod factsheet {
        pub use crate::factsheet::ActionParameter;
        pub use crate::factsheet::ActionScope;
        pub use crate::factsheet::AgvAction;
        pub use crate::factsheet::AgvClass;
        pub use crate::factsheet::AgvGeometry;
        pub use crate::factsheet::AgvKinematic;
        pub use crate::factsheet::Data;
        pub use crate::factsheet::DockingDirection;
        pub use crate::factsheet::Envelopes2d;
        pub use crate::factsheet::Envelopes3d;
        pub use crate::factsheet::Factsheet;
        pub use crate::factsheet::LoadSet;
        pub use crate::factsheet::LoadSpecification;
        pub use crate::factsheet::LocalizationType;
        pub use crate::factsheet::MaxArrayLens;
        pub use crate::factsheet::MaxStringLens;
        pub use crate::factsheet::NavigationType;
        pub use crate::factsheet::OptionalParameter;
        pub use crate::factsheet::PhysicalParameters;
        pub use crate::factsheet::PolygonPoint;
        pub use crate::factsheet::Position;
        pub use crate::factsheet::ProtocolFeatures;
        pub use crate::factsheet::ProtocolLimits;
        pub use crate::factsheet::Support;
        pub use crate::factsheet::Timing;
        pub use crate::factsheet::TypeSpecification;
        pub use crate::factsheet::ValueDataType;
        pub use crate::factsheet::WheelDefinition;
        pub use crate::factsheet::WheelType;
    }

    pub mod instant_actions {
        pub use crate::instant_actions::InstantActions;
    }

    pub mod order {
        pub use crate::order::Edge;
        pub use crate::order::Node;
        pub use crate::order::Order;
        pub use crate::order::OrientationType;
    }

    pub mod state {
        pub use crate::state::ActionState;
        pub use crate::state::ActionStatus;
        pub use crate::state::BatteryState;
        pub use crate::state::EStop;
        pub use crate::state::EdgeState;
        pub use crate::state::Error;
        pub use crate::state::ErrorLevel;
        pub use crate::state::ErrorReference;
        pub use crate::state::InfoLevel;
        pub use crate::state::InfoReference;
        pub use crate::state::Information;
        pub use crate::state::Load;
        pub use crate::state::NodeState;
        pub use crate::state::OperatingMode;
        pub use crate::state::SafetyState;
        pub use crate::state::State;
    }

    pub mod visualization {
        pub use crate::visualization::Visualization;
    }

    pub mod action {
        pub use crate::action::Action;
        pub use crate::action::ActionParameter;
        pub use crate::action::ActionParameterValue;
    }
}
