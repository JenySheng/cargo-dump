## Download the repository:
- Method 1: directly clone the repository in VScode:
  
	Open VScode and use the “Clone Git Repository”. Copy and paste the repository URL [https://github.com/JenySheng/cargo-dump](https://github.com/JenySheng/cargo-dump) and then select your preferred repository destination. Then we can open the files in the VScode and run it.

- Method 2: Using the terminal:
	Open a new terminal and select your preferred repository destination. For example: if I want to save the folder in the Desktop, I can use the following commands by steps:

    1. ~/Desktop

    2. git clone https://github.com/JenySheng/cargo-dump

  Then, I can find that folder on my desktop and open it in the preferred UI, such as the VScode. 

## Download front-end related things:
We used Yew to display our front-end. There are the following things that need to be installed. Following the steps below to install them in your terminal:
      
      cargo install trunk

      rustup target add wasm32-unknown-unknown

  After this, we can successfully open the whole folder and run the front-end

## How to successfully get a prediction for your cards? Use following commands in your terminal:


  - `trunk serve` (then you can open a web such as google and use “http://localhost:8080/”). Then you can see our UI design of our webpage(if you want to get your prediction in a user friendly website)

  - open a new terminal, type `cargo run`. Then you can follow the instructions(please follow that or you will get “panicked”) in the terminal. You can get the prediction in the terminal.
If you want to start a new group of cards, you need to close the terminal and open a new terminal and type `cargo run`. 
