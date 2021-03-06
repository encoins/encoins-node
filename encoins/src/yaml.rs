extern crate yaml_rust;
use yaml_rust::yaml::{Hash, Yaml, YamlLoader};
use std::fs;

/// Transform the yaml file into a Hash table
pub fn yaml_to_hash(file: &str) -> Hash 
{
    // Load the yaml file into a str
    let str_yaml: &str = &fs::read_to_string(file)
        .expect("file net_config.yml not found at {}, be sure to be in encoins-node/encoins")[..];

    // Transform the str into a Yaml hash table
    let vec_yaml: Vec<Yaml> = YamlLoader::load_from_str(str_yaml)
        .expect("Failed to convert the content of network configuration file into a yaml table");

    vec_yaml[0].as_hash()
        .expect("Syntax problem in yaml file")
        .clone()
}

/// Access hash[key1][key2]
fn read_yaml(hash: &Hash, key1: &str, key2: &str) -> Yaml 
{
    // Access to the nested hash_table
    let key1_yaml: Yaml = Yaml::String(key1.to_string());
    let hash_nested: &Hash = hash[&key1_yaml].as_hash()
        .expect("Syntax problem in yaml file");
    
    // Access to the value
    let key2_yaml: Yaml = Yaml::String(key2.to_string());
    hash_nested[&key2_yaml]
        .clone()
}

/// Read the content of server{i} section
pub fn read_server_address(hash_net_config: &Hash, i: u32) -> (String, u16, u16) 
{
    let server_i: String = "server".to_owned() + &i.to_string();

    let address_yaml: Yaml = read_yaml(hash_net_config, &server_i, "address");
    let address: String = address_yaml.into_string()
        .expect("In yaml file, one ip adress is not of string type");

    let port_server: u16 = read_yaml(hash_net_config, &server_i, "port_server")
        .as_i64()
        .expect("In yaml file, one port adress is not of int type")
        as u16;

    let port_client: u16 = read_yaml(hash_net_config, &server_i, "port_client")
        .as_i64()
        .expect("In yaml file, one port adress is not of int type")
        as u16;
    
    (address, port_server, port_client)
}

/// Read the content of parameters section
pub fn read_network_parameters(hash_net_config: &Hash) -> u32 
{
    let nb_servers: u32 = read_yaml(hash_net_config, "parameters", "nb_servers")
    .as_i64()
    .expect("In yaml file, nb_servers is not of int type")
    as u32;

    nb_servers
}