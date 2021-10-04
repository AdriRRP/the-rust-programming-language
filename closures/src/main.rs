use std::thread;
use std::time::Duration;

/*
 * We work at a startup that’s making an app to generate custom exercise
 * workout plans. The backend is written in Rust, and the algorithm that
 * generates the workout plan takes into account many factors, such as the app
 * user’s age, body mass index, exercise preferences, recent workouts, and an
 * intensity number they specify. The actual algorithm used isn’t important in
 * this example; what’s important is that this calculation takes a few seconds.
 * We want to call this algorithm only when we need to and only call it once so
 * we don’t make the user wait more than necessary.
 *
 * We’ll simulate calling this hypothetical algorithm with the function
 * simulated_expensive_calculation shown below, which will print
 * calculating slowly..., wait for two seconds, and then return whatever number
 * we passed in.
 *
 */

fn simulated_expensive_calculation(intensity: u32) -> u32 {
    println!("calculating slowly...");
    thread::sleep(Duration::from_secs(2));
    intensity
}

/*
 * Next is the main function, which contains the parts of the workout
 * app important for this example. This function represents the code that the
 * app will call when a user asks for a workout plan. Because the interaction
 * with the app’s frontend isn’t relevant to the use of closures, we’ll
 * hardcode values representing inputs to our program and print the outputs.
 *
 * The required inputs are these:
 *
 *  - An intensity number from the user, which is specified when they request a
 *    workout to indicate whether they want a low-intensity workout or a
 *    high-intensity workout
 *
 *  - A random number that will generate some variety in the workout plans
 *
 * The output will be the recommended workout plan. 
 *
 */

fn main() {
    let simulated_user_specified_value = 10;
    let simulated_random_number = 7;

    generate_workout(simulated_user_specified_value, simulated_random_number);
}

/*
 * Now that we have the context, let’s get to the algorithm. The function
 * generate_workout contains the business logic of the app that we’re most
 * concerned with in this example. The rest of the code changes in this example
 * will be made to this function.
 *
 */

fn generate_workout(intensity: u32, random_number: u32) {
    if intensity < 25 {
        println!(
            "Today, do {} pushups!",
            simulated_expensive_calculation(intensity)
        );
        println!(
            "Next, do {} situps!",
            simulated_expensive_calculation(intensity)
        );
    } else {
        if random_number == 3 {
            println!("Take a break today! Remember to stay hydrated!");
        } else {
            println!(
                "Today, run for {} minutes!",
                simulated_expensive_calculation(intensity)
            );
        }
    }
}
