/** ***********************************
* *[author] Diogo André (git-hub : das-dias)
* *[date] 23-03-2022
* *[filename] layer.rs
* *[summary]    Implements the data structures to retrieve the LAYER instance DRC rules from the Tech file
* *[notes]      The technology file (.tlef) is constituted by Layer (Rules), Via (Rules) and Via-rules
* *             
* *             The layout layer table file (.ltab.tlef) is constituted by Layers and their corresponding 
* *             geometry file indexes.
* ***********************************
*/
use std::collections::HashMap;
/**
* *[name] LayerRule
* *[description] Individual Layer DRC Rule container
* [variables]
* @par name         (&'static str) : name of the LAYER rule instance
* @par str_val      (&'static str) : string value of the rule
* @par param        (f32) : numeric value of the rule
* @par vec_param    (Vec<(f32,f32)>) : vector cartesian parameter of the rule
*/
#[derive(Debug)]
pub struct LayerRule<'lr>
{
    name:       &'static str,
    str_val:    &'static str,
    param:      f32,
    vec_param:  Vec<(f32,f32)>,

}

/**
* *[name] LayerTableRule
* *[description] TABLE instance data structure
* [variables]
* @par name     (&'static str) : name of the TABLE instance
* @par str_val  (&'static str) : string value identifying the type of the table
* @par entries  (Vec<LayerRule>) : entries of the rule table as a vector of LayerRule's
*/
#[derive(Debug)]
pub struct LayerTableRule<'ltr>
{
    name:       &'static str,
    str_val:    &'static str,
    entries:    Vec<LayerRule>
}

/**
* *[name] LAYER rules container data structure
* *[description] 
* [variables]
* @par layer_name   (&'static str) : name of the layer
* @par rules        (Vec<LayerTableRule>) : Collection of rules of the layer instance
* ?NOTE: a Layer Rule is a Layer Rule Table with a single entry
*/
#[derive(Debug)]
pub struct Layer<'l>
{
    layer_name: &'static str,
    rules:      Vec<LayerTableRule>,
}

/**
* *[name] LayerRuleLibrary
* *[description] Library of technology LAYER instances
* [variables]
* @par layer_rules (HashMap<&str,Layer>) : map of {layer_name : Layer rules instance} tuples
*/
pub struct LayerRuleLibrary<'lrl>
{
    layer_rules: HashMap<&'static str, Layer>
}

/**
* *[name] LayerToLayoutTable
* *[description]    Table saving the associated technology 
* *                 layers with each layout layer index
* !Deprecated: The LayerToLayoutTable is implemented with a HashMap
* [variables]
* @par entries (Vec<(str, unsigned int 8bit)>) : vector of ("layer name", layer_index) tuples
*/
#[derive(Debug)]
pub struct LayerToLayoutTable<'l2l>
(
    entries: Vec<(&'static str, u8)>
)