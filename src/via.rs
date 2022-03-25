/** ***********************************
* *[author] Diogo André (git-hub : das-dias)
* *[date] 23-03-2022
* *[filename] via.rs
* *[summary]    Implements the data structures to retrieve the VIA and VIARULE instance DRC rules from the Tech file
* *[notes]      The technology file (.tlef) is constituted by Layer (Rules), Via (Rules) and Via-rules
* *             
* *             The layout layer table file (.ltab.tlef) is constituted by Layers and their corresponding 
* *             geometry file indexes.
* ***********************************
*/
/*
* *[name] pxy
* *[description] 2D cartesian point
* ![deprecated] WILL BE DEPRECATED TO THE USE OF TUPLES
* [variables]
* @par <param name> (type) : <first var. description>
*/
#[derive(Debug)]
pub struct pxy<'p>
{
    x: f32,
    y: f32
}

/**
* *[name] ViaRule
* *[description] Via DRC individual rule container
* [variables]
* @par name     (&'static str) : string identifier of the rule
* @par layer    (&'static str) : name of the associated layer
* @par params   (Vec<(f32,f32)>) : vector of cartesian parameters for the rule
*/
#[derive(Debug)] // enable the print of this structure in debug mode
pub struct ViaRule<'vr> // VIA DRC RULES
{
    name:   &'static str,
    layer:  &'static str,
    params: Vec<(f32,f32)>,
}

/**
* *[name] Via
* *[description] VIA rules container data structure
* [variables]
* @par via_model_name   (&'static str) : name of the associated via model
* @par typ              (&'static str) : name of the type of the via instance (VIA / VIARULE)
* @par mode             (&'static str) : name of the mode of via instance (DEFAULT, GENERATE)
* @par rules            (&'static str) : rules associated with this VIA instance 
*/
#[derive(Debug)]
pub struct Via<'v> // VIA INSTANCE
{
    via_model_name: &'static str, 
    typ:            &'static str, 
    mode:           &'static str, 
    rules:          Vec<ViaRule>,
}

/**
* *[name] ViaRuleLibrary
* *[description] Library of technology VIA and VIARULE instances
* [variables]
* @par via_rules    (HashMap<&str, Via>) : dictionary of {via_model_name : VIARULE rules instance} tuples
* @par vias         (HashMap<&str, Via>) : dictionary of {via_model_name : VIA rules instance} tuples
*/
pub struct ViaRuleLibrary<'vrl>
{
    via_rules:  HashMap<&'static str,Via>,
    vias:       HashMap<&'static str,Via>,
}