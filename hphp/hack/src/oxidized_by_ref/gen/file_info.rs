// Copyright (c) Facebook, Inc. and its affiliates.
//
// This source code is licensed under the MIT license found in the
// LICENSE file in the "hack" directory of this source tree.
//
// @generated SignedSource<<98622815728a97c3a55e6840dd73b4aa>>
//
// To regenerate this file, run:
//   hphp/hack/src/oxidize_regen.sh

use arena_trait::TrivialDrop;
use no_pos_hash::NoPosHash;
use ocamlrep_derive::FromOcamlRepIn;
use ocamlrep_derive::ToOcamlRep;
use serde::Serialize;

#[allow(unused_imports)]
use crate::*;

pub use prim_defs::*;

pub use oxidized::file_info::Mode;

pub use oxidized::file_info::NameType;

/// We define two types of positions establishing the location of a given name:
/// a Full position contains the exact position of a name in a file, and a
/// File position contains just the file and the type of toplevel entity,
/// allowing us to lazily retrieve the name's exact location if necessary.
#[derive(
    Clone,
    Copy,
    Debug,
    Eq,
    FromOcamlRepIn,
    Hash,
    NoPosHash,
    Ord,
    PartialEq,
    PartialOrd,
    Serialize,
    ToOcamlRep
)]
pub enum Pos<'a> {
    Full(&'a pos::Pos<'a>),
    File(
        &'a (
            oxidized::file_info::NameType,
            &'a relative_path::RelativePath<'a>,
        ),
    ),
}
impl<'a> TrivialDrop for Pos<'a> {}

#[derive(
    Clone,
    Debug,
    Eq,
    FromOcamlRepIn,
    Hash,
    NoPosHash,
    Ord,
    PartialEq,
    PartialOrd,
    Serialize,
    ToOcamlRep
)]
pub struct Id<'a>(pub Pos<'a>, pub &'a str);
impl<'a> TrivialDrop for Id<'a> {}

/// The hash value of a decl AST.
/// We use this to see if two versions of a file are "similar", i.e. their
/// declarations only differ by position information.
pub type HashType<'a> = Option<isize>;

/// The record produced by the parsing phase.
#[derive(
    Clone,
    Debug,
    Eq,
    FromOcamlRepIn,
    Hash,
    NoPosHash,
    Ord,
    PartialEq,
    PartialOrd,
    Serialize,
    ToOcamlRep
)]
pub struct FileInfo<'a> {
    pub hash: &'a HashType<'a>,
    pub file_mode: Option<oxidized::file_info::Mode>,
    pub funs: &'a [&'a Id<'a>],
    pub classes: &'a [&'a Id<'a>],
    pub record_defs: &'a [&'a Id<'a>],
    pub typedefs: &'a [&'a Id<'a>],
    pub consts: &'a [&'a Id<'a>],
    /// None if loaded from saved state
    pub comments: Option<&'a [(&'a pos::Pos<'a>, Comment<'a>)]>,
}
impl<'a> TrivialDrop for FileInfo<'a> {}

pub use oxidized::file_info::Names;

pub use oxidized::file_info::Saved;

#[derive(
    Clone,
    Debug,
    Eq,
    FromOcamlRepIn,
    Hash,
    NoPosHash,
    Ord,
    PartialEq,
    PartialOrd,
    Serialize,
    ToOcamlRep
)]
pub struct Diff<'a> {
    pub removed_funs: s_set::SSet<'a>,
    pub added_funs: s_set::SSet<'a>,
    pub removed_classes: s_set::SSet<'a>,
    pub added_classes: s_set::SSet<'a>,
    pub removed_types: s_set::SSet<'a>,
    pub added_types: s_set::SSet<'a>,
    pub removed_consts: s_set::SSet<'a>,
    pub added_consts: s_set::SSet<'a>,
}
impl<'a> TrivialDrop for Diff<'a> {}
