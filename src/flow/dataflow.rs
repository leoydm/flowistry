use anyhow::Result;
use rustc_middle::{mir::*, ty::TyCtxt};
use rustc_mir::dataflow::{fmt::DebugWithContext, Analysis, AnalysisDomain, Forward};
use std::fmt;

use crate::core::{
  aliases::Aliases, control_dependencies::ControlDependencies, place_set::PlaceMatrix,
};

pub type FlowDomain = PlaceMatrix;

pub struct FlowAnalysis<'a, 'tcx> {
  tcx: TyCtxt<'tcx>,
  body: &'a Body<'tcx>,
  control_dependencies: &'a ControlDependencies,
  aliases: &'a Aliases<'tcx>,
}

impl FlowAnalysis<'a, 'tcx> {
  pub fn new(
    tcx: TyCtxt<'tcx>,
    body: &'a Body<'tcx>,
    aliases: &'a Aliases<'tcx>,
    control_dependencies: &'a ControlDependencies,
  ) -> Self {
    FlowAnalysis {
      tcx,
      body,
      aliases,
      control_dependencies,
    }
  }
}

impl AnalysisDomain<'tcx> for FlowAnalysis<'a, 'tcx> {
  type Domain = FlowDomain;
  type Direction = Forward;
  const NAME: &'static str = "FlowAnalysis";

  fn bottom_value(&self, _body: &Body<'tcx>) -> Self::Domain {
    FlowDomain::new(&self.aliases.place_domain)
  }

  fn initialize_start_block(&self, _: &Body<'tcx>, _: &mut Self::Domain) {}
}

impl Analysis<'tcx> for FlowAnalysis<'a, 'tcx> {
  fn apply_statement_effect(
    &self,
    state: &mut Self::Domain,
    statement: &Statement<'tcx>,
    location: Location,
  ) {
    // todo!()
  }

  fn apply_terminator_effect(
    &self,
    state: &mut Self::Domain,
    terminator: &Terminator<'tcx>,
    location: Location,
  ) {
    // todo!()
  }

  fn apply_call_return_effect(
    &self,
    state: &mut Self::Domain,
    block: BasicBlock,
    func: &Operand<'tcx>,
    args: &[Operand<'tcx>],
    return_place: Place<'tcx>,
  ) {
  }
}

impl DebugWithContext<FlowAnalysis<'_, '_>> for FlowDomain {
  fn fmt_with(&self, ctxt: &FlowAnalysis, f: &mut fmt::Formatter<'_>) -> fmt::Result {
    todo!()
  }
}
