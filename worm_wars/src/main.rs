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

        // return
        SimData {
            popSeed: 0,
            S: 0, I: 0, R: 0,
            sToI: 0.0, iToR: 0.0, sToR: 0.0,
            updateRates: HashMap::new(),
        }
    }

    fn parse_conditions(&mut self, st: &str) {
        let mut initConditions = st.split_whitespace();

        self.popSeed = initConditions.next().unwrap().parse().unwrap();
        self.I = initConditions.next().unwrap().parse().unwrap();
        self.sToI = initConditions.next().unwrap().parse().unwrap();
        self.iToR = initConditions.next().unwrap().parse().unwrap();
        self.sToR = initConditions.next().unwrap().parse().unwrap();
        self.S = self.popSeed as i32 - self.I;
    }
}


fn main() {
    let mut simData = SimData::new();
    let inputFile = File::open("simConditions.txt").unwrap();
    let mut inputFile = BufReader::new(&inputFile);

    // Get total generations
    // Thanks to /u/svgwrk for this syntax
    let totalGenerations: u32 = {
        let mut temp = String::new();
        let _ = inputFile.read_line(&mut temp);

        // return
        temp.trim().parse().unwrap()
    };


    // Setup initial conditions
    let mut initConditions = String::new();
    let _ = inputFile.read_line(&mut initConditions);
    simData.parse_conditions(&initConditions);


    // Add updated rates
    for line in inputFile.lines() {
        let line = line.unwrap();
        let mut line = line.split_whitespace();

        // Dear lord this is ugly
        simData.updateRates.insert(line.next().unwrap().parse().unwrap(), 
        (line.next().unwrap().parse().unwrap(), line.next().unwrap().parse().unwrap(), 
         line.next().unwrap().parse().unwrap()));
    }

    
    // The actual sim, starting with/reporting 0-day.
    for gen in 0..(totalGenerations + 1) {
        println!("Generation {}:\n  S: {}, I: {}, R: {}", gen, simData.S, simData.I, simData.R);
        
        if let Some(&(newSToI, newIToR, newSToR)) = simData.updateRates.get(&gen) {
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