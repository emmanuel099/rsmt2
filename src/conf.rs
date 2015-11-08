// Copyright 2015 Adrien Champion. See the COPYRIGHT file at the top-level
// directory of this distribution.
//
// Licensed under the Apache License, Version 2.0 <LICENSE-APACHE or
// http://www.apache.org/licenses/LICENSE-2.0> or the MIT license
// <LICENSE-MIT or http://opensource.org/licenses/MIT>, at your
// option. This file may not be copied, modified, or distributed
// except according to those terms.

/*! Solver configuration, contains backend solver specific info. */

use common::UnexSmtRes ;

use self::SolverStyle::* ;

/** A configuration item is either a keyword or unsupported. */
pub type ConfItem = Result<& 'static str, UnexSmtRes> ;

/** Value of an unsupported configuration item. */
#[inline(always)]
fn unsupported() -> ConfItem { Err(UnexSmtRes::Unsupported) }
/** Keyword for a configuration item. */
#[inline(always)]
fn supported(keyword: & 'static str) -> ConfItem { Ok(keyword) }


/** Solver styles. */
#[derive(Debug)]
pub enum SolverStyle {
  /** Z3-style smt solver. */
  Z3,
  /** CVC4-style smt solver. */
  CVC4,
}

impl SolverStyle {
  /** Default configuration for a solver style. */
  pub fn default(self) -> SolverConf {
    match self {
      Z3 => SolverConf {
        style: self,
        options: vec![
          "-in", "-smt2"
        ],
        parse_success: false,
        unsat_cores: false,
        check_sat_assuming: supported("check-sat"),
      },
      CVC4 => SolverConf {
        style: self,
        options: vec![
          "--smtlib-strict", "-qqqqq", "--interactive"
        ],
        parse_success: false,
        unsat_cores: false,
        check_sat_assuming: unsupported(),
      },
    }
  }
}


/** Configuration and solver specific info. */
pub struct SolverConf {
  /** Solver style. */
  style: SolverStyle,
  /** Options to call the solver with. */
  options: Vec<& 'static str>,
  /** Parse success. */
  parse_success: bool,
  /** Triggers unsat-core production. */
  unsat_cores: bool,
  /** Keyword for check-sat with assumptions. */
  check_sat_assuming: ConfItem,
}

impl SolverConf {
  /** Creates a new z3-like solver configuration. */
  #[inline(always)]
  pub fn z3() -> Self { Z3.default() }

  /** Creates a new CVC4-like solver configuration. */
  #[inline(always)]
  pub fn cvc4() -> Self { CVC4.default() }


  /** Solver style. */
  #[inline(always)]
  pub fn style(& self) -> & SolverStyle { & self.style }

  /** Options of the configuration. */
  #[inline(always)]
  pub fn get_options(& self) -> & [& 'static str] { & self.options }

  /** Indicates if parse success is active. */
  #[inline(always)]
  pub fn get_parse_success(& self) -> bool { self.parse_success }

  /** Keyword for check-sat with assumptions. */
  #[inline(always)]
  pub fn get_check_sat_assuming(& self) -> & ConfItem {
    & self.check_sat_assuming
  }

  /** Adds an option to the configuration. */
  #[inline(always)]
  pub fn option(mut self, o: & 'static str) -> Self {
    self.options.push(o) ;
    self
  }

  /** Activates parse sucess. */
  #[inline(always)]
  #[cfg(not(no_parse_success))]
  pub fn print_success(mut self) -> Self {
    self.parse_success = true ;
    self
  }

  /** Activates unsat-core production. */
  #[inline(always)]
  pub fn unsat_cores(mut self) -> Self {
    self.unsat_cores = true ;
    self
  }


  // /** Creates a solver instance. */
  // #[inline(always)]
  // pub fn mk<Parser>(
  //   self, cmd: Command, parser: Parser
  // ) -> io::Result<Solver<Parser>> {
  //   Solver::mk(cmd, self, parser)
  // }
}