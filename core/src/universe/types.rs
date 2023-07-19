use super::files;
use num::complex::Complex;
use serde::{Deserialize, Serialize};
use serde_with::serde_as;
use std::collections::HashMap;

#[derive(Serialize, Deserialize, Clone, Hash, Eq, PartialEq, Ord, PartialOrd, Debug)]
pub struct Coordinates {
    pub x: i32,
    pub y: i32,
}

#[derive(Serialize, Deserialize)]
#[serde(remote = "Complex")]
pub struct ComplexDef<T> {
    /// Real portion of the complex number
    pub re: T,
    /// Imaginary portion of the complex number
    pub im: T,
}

// The HashMap bool value in the living_cells attribute
// is true if this cell has already been computed during the current step
// We decided to do that to optimize memory but it needs to be reviewed later
// use a struct would be better for readability
#[serde_as]
#[derive(Serialize, Deserialize, Clone, Debug)]
pub struct Configuration {
    #[serde(with = "ComplexDef")]
    pub amplitude: Complex<f64>,

    #[serde_as(as = "Vec<(_, _)>")]
    pub living_cells: HashMap<Coordinates, bool>,
}

pub type State = Vec<Configuration>;

#[derive(Serialize, Deserialize, Debug)]
pub struct YamlRulesComplex {
    pub re: String,
    pub im: String,
}

pub type YamlRules = HashMap<i32, HashMap<i32, YamlRulesComplex>>;
pub type Rules = HashMap<i32, HashMap<i32, Complex<f64>>>;

pub fn yaml_rules_to_universe_rules(
    yaml_rules: YamlRules,
) -> Result<Rules, evalexpr::EvalexprError> {
    let mut rules: Rules = Rules::new();

    for (key, value) in yaml_rules {
        let mut rule = HashMap::new();

        for (k, v) in value {
            let context = evalexpr::math_consts_context!()?;
            let re: f64 = evalexpr::eval_number_with_context(&v.re, &context)?;
            let im: f64 = evalexpr::eval_number_with_context(&v.im, &context)?;

            rule.insert(k, Complex::new(re, im));
        }

        rules.insert(key, rule);
    }

    Ok(rules)
}

// The is_even_step attribute is used to determine the square in which
// the rules of the universe apply for a given living cell
// It is true if the universe is in an even step and false othrerwise
#[derive(Clone, Debug)]
pub struct Universe {
    pub state: State,
    pub combined_state: HashMap<Coordinates, f64>,
    pub is_even_step: bool,
    pub rules: Rules,
    pub step_count: usize,
}

impl Universe {
    pub fn new() -> Result<Self, Box<dyn std::error::Error>> {
        // Unique configuration with no living cell
        let configuration = Configuration {
            amplitude: Complex::new(1., 0.),
            living_cells: HashMap::new(),
        };
        let state = vec![configuration];
        let rules = get_default_rules()?;
        let step_count = 0;
        Ok(Self {
            state,
            combined_state: HashMap::new(),
            is_even_step: true,
            rules,
            step_count,
        })
    }

    pub fn new_from_files(state_file: &str) -> Result<Self, Box<dyn std::error::Error>> {
        let state = files::get_state_from_file(state_file)?;
        let rules = get_default_rules()?;
        let step_count = 0;
        let mut universe = Self {
            state,
            combined_state: HashMap::new(),
            is_even_step: true,
            rules,
            step_count,
        };
        universe.compute_combined_state();
        Ok(universe)
    }

    pub fn new_from_str(content: &str) -> Result<Self, Box<dyn std::error::Error>> {
        let state: State = serde_json::from_str(content)?;
        let rules = get_default_rules()?;
        let step_count = 0;
        let mut universe = Self {
            state,
            combined_state: HashMap::new(),
            is_even_step: true,
            rules,
            step_count,
        };
        universe.compute_combined_state();
        Ok(universe)
    }
}

pub fn get_default_rules() -> Result<Rules, Box<dyn std::error::Error>> {
    let yaml_rules = files::get_rules_from_file("./core/fixtures/rules/default_rules.yaml")?;
    let rules = yaml_rules_to_universe_rules(yaml_rules)?;

    Ok(rules)
}
