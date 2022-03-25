/** ***********************************
* *[author] Diogo André (git-hub : das-dias)
* *[date] 23-03-2022
* *[filename] tech.rs
* *[summary]    Implements the data structures to retrieve from the technology file
* *[notes]      The technology file (.tlef) is constituted by Layer (Rules), Via (Rules) and Via-rules
* *             
* *             The layout layer table file (.ltab.tlef) is constituted by Layers and their corresponding 
* *             geometry file indexes.
* ***********************************
*/

use std::collections::HashMap;
mod via;
mod layer;

/**
* *[name] UnitRule
* *[description] Unit or Header rule of the technology file (it is used for both)
* ?NOTE: Unit rules will always hold a non-null val and a null unit, while Header rules can have both values
* [variables]
* @par name (&'static str) : name of the unit rule
* @par unit (&'static str) : name of the unit of the physical quantity
* @par val  (f32) : numeric value of the unit rule
*/
pub struct HeaderUnitRule<'hur>
{
    name:   &'static str,
    unit:   &'static str,
    val:    f32
}

/**
* *[name] UnitRuleLibrary
* *[description] Unit rule library of the technology file
* [variables]
* @par name (&'static str) : name of the unit rule
* @par units (HashMap<&'static str, HeaderUnitRule>) : dictionary of {unit name : Unit rule} tuples
*/
pub struct UnitRuleLibrary<'ul>
{
    name:   &'static str,
    units:  HashMap<&'static str, HeaderUnitRule>
}

/**
* *[name] HeaderRuleLibrary
* *[description] Unit rule library of the technology file
* [variables]
* @par header_rule_lib (HashMap<&'static str, HeaderUnitRule>) : dictionary of {header rule name : Header rule} tuples
*/
pub struct HeaderRuleLibrary<'hr>
{
    header_rule_lib:    HashMap<&'static str, HeaderUnitRule> 
}

/**
* *[name] Technology
* *[description] Technology data container
* [variables]
* @par name                     (&'static str) : name of the technology
* @par version                  (&'static str) : version of the technology file
* @par description              (&'static str) : description of the technology
* @par group                    (&'static str) : name of the group that develops the technology
* @par grain_name               (&'static str) : name of the technology grain
* @par default_base_path        (&str) : base path to the technology files
* @par explicit_base_path       (&str) : explicit path to the technology files
* @par load_layout_options      (bool) : boolean indicating that the layout rules must be read
* @par save_layout_options      (bool) : boolean indicating that the layout rules must be saved
* @par l2ltab_explicit_path     (&str) : explicit path of the layer to layout index table
* @par add_other_layers         (bool) : boolean indicating that adtional layer to layout layer indexes must be added
* !@par persisted               (?) : ?
* @par read_only                (bool) : boolean indicating the opening mode of the technology file
* @par pr_cells_path            (&str) : base path of the primitive geometry cell models
* @par pr_models_path           (&str) : base path of the primitive spice models
* @par header_rules             (HeaderRuleLibrary) : technology file rules contained in its header
* @par units                    (UnitRuleLibrary)   : unit rules of the technology file
* @par layer_rule_lib           (layer::LayerRuleLibrary) : library of LAYER rules instances within the tech file
* @par via_rule_lib             (via::ViaRuleLibrary) : library of VIA and VIARULE rules instances within the tech file
* @par layer_to_layout_tab      (HashMap<&str, u8>) : Table saving the associated technology 
* @                                                 layers with each layout layer index
*/
pub struct Technology<'t>
{
    name:                   &'static str,
    description:            &'static str,
    group:                  &'static str,
    grain_name:             &'static str,
    default_base_path:      &str,
    explicit_base_path:     &str,
    load_layout_options:    bool,
    save_layout_options:    bool,
    l2ltab_explicit_path:   &str,
    add_other_layers:       bool,
    read_only:              bool,
    pr_cells_path:          &str,
    pr_models_path:         &str,
    header_rules:           HeaderRuleLibrary,
    units:                  UnitRuleLibrary,
    layer_rule_lib:         LayerRuleLibrary,
    via_rule_lib:           ViaRuleLibrary,
    layer_to_layout_table:  HashMap<&'static str, u8>,
}

/**
* *[name] TechnologyLibrary
* *[description]    Library of available technologies for the engineer 
* *                 (to be recovered from JSON/YAML at the beggining of Speedster)
* [variables]
* @par techs (HashMap<&'static str, Technology>) : dictionary of {tech name : Technology data container} tuples
*/
pub struct TechnologyLibrary<'tl>
{
    techs:  HashMap<&'static str, Technology>
}