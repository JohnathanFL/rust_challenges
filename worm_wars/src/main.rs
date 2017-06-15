// I apologize to any rust_snake_case lovers.
#![allow(non_snake_case)]

use std::io::{BufRead, BufReader};
use std::fs::File;
use std::collections::HashMap;


struct SimData {
    popSeed: u32, // Initial total systems.
    S: i32, // Number clean.
    I: i32, // Number infected.
    R: i32, // Number of immunized.
    sToI: f32, // Rate at which clean systems become infected.
    iToR: f32, // Rate at which infected are cured.
    sToR: f32, // Rate at which clean are immunized.

    updateRates: HashMap<u32, (f32, f32, f32)>, // Each set of updates, mapped to the timestep
                                                // at which they appear.
}

impl SimData {
    fn new() -> SimData {
        return SimData {
            popSeed: 0,
            S: 0, I: 0, R: 0,
            sToI: 0.0, iToR: 0.0, sToR: 0.0,
            updateRates: HashMap::new(),
        };
    }
}

fn main() {
    let mut simData = SimData::new();
    let totalGenerations: u32;
    let inputFile = File::open("simConditions.txt").unwrap();
    let mut inputFile = BufReader::new(&inputFile);

    // Get total generations
    let mut temp = String::new();
    inputFile.read_line(&mut temp);
    temp = (&temp[..]).trim().to_string();
    totalGenerations = temp.to_string().parse::<i32>().unwrap() as u32;


    // Setup initial conditions
    let mut initConditions = String::new();
    inputFile.read_line(&mut initConditions);
    let mut initConditions = initConditions.split_whitespace();

    // There must be a better way to do this part... :(
    simData.popSeed = initConditions.next().unwrap().parse::<u32>().unwrap();
    simData.I = initConditions.next().unwrap().parse::<i32>().unwrap();
    simData.sToI = initConditions.next().unwrap().parse::<f32>().unwrap();
    simData.iToR = initConditions.next().unwrap().parse::<f32>().unwrap();
    simData.sToR = initConditions.next().unwrap().parse::<f32>().unwrap();
    simData.S = simData.popSeed as i32 - simData.I;


    // Add updated rates
    let mut temp = String::new();
    for line in inputFile.lines() {
        temp = line.unwrap();
        let mut line = temp.split_whitespace();

        // Dear lord this is ugly
        simData.updateRates.insert(line.next().unwrap().parse::<u32>().unwrap(), 
        (line.next().unwrap().parse::<f32>().unwrap(), line.next().unwrap().parse::<f32>().unwrap(), 
         line.next().unwrap().parse::<f32>().unwrap()));
    }

    
    // The actual sim, starting with/reporting 0-day.
    for gen in 0..(totalGenerations + 1) {
        println!("Generation {}:\n  S: {}, I: {}, R: {}", gen, simData.S.to_string(), simData.I.to_string(), simData.R.to_string());
        
        if simData.updateRates.contains_key(&gen) {
            let (newSToI, newIToR, newSToR) = simData.updateRates[&gen];

            simData.sToI = newSToI;
            simData.iToR = newIToR;
            simData.sToR = newSToR;
            println!("Updated sim rates to: {:?}", simData.updateRates[&gen]);
        }

        // I use ceil because I assume we can't have partial infections.
        let numSToI = ((simData.S as f32) * simData.sToI).ceil() as i32;
        let numIToR = ((simData.I as f32) * simData.iToR).ceil() as i32;
        let numSToR = ((simData.S as f32) * simData.sToR).ceil() as i32;

        
        simData.S -= numSToR + numSToI; // Infections and immunizations remove from S.
        simData.I += numSToI - numIToR; // Infections add to I but immunizations remove from it.
        simData.R += numSToR + numIToR; // All immunizations add to R.

        // Make sure we don't go negative
        if simData.S <= 0 || simData.I <= 0 {
            break;
        }
    }
}