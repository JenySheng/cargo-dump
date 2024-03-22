# 🎉Welcome to cargo-dump!
🦀Group name: Cargo Dump

## **Team Members 👩‍💻👩‍💻👩‍💻**

Jeny Sheng (jenys2)

Ivy Gu (lixuang2)

Yanlin Tao(tao17)

## ♠️ ♣️ ♥️ ♦️Project introduction

Our project aims to develop a web application using Rust that assists players in analysing the winning possibilities of various card combinations in Texas Hold'em Poker.  

For our project, our goal is letting people input their hand and community cards and receive the likelihood of winning based on different scenarios. Meanwhile, we will provide what cards exactly needed to satisfy certain kind of combination (such as the flush, suit, etc.) to win and the chance for them to get that card. 

We chose to work on this project because Texas Hold'em Poker is a popular and widely-played card game that involves strategic decision-making and probability analysis. By developing such a webpage, we can improve our gameplay and decision making. 

## 📌Technical Overview:
- Technical description
  
  Roadmaps for webpage and code 

![image](https://github.com/JenySheng/cargo-dump/assets/143043541/da322d62-1f4e-4f4d-9f69-71bcc648e12a)
![image](https://github.com/JenySheng/cargo-dump/assets/143043541/3a14bb2d-acbd-41b2-a8ff-0b1723584c11)

The above 2 road maps outline the basic process of the game and how we aim to go through the game alongside the player, and give a new possibility every time a new card is revealed.

For the web page, our tentative approach is to use Rocket and/or similar web frameworks to make a webpage with rust.
    
    Frontend choices: Yew, Stdweb

    Backend choices: Rocket, Axum

  Other thing to learn
  
    WebAssembly

- Checkpoint#1

Finish the basic code of calculation. Specifically, to write a code that calculates, after each draw of the player, the possibility of each result(high card, flush, etc.) and the possible cards needed for that combination.Learning basic knowledge about website development.

- Checkpoint#2

Finish the design of webpage so that users can input their own cards, then the community cards one by one and have the possibility of each combination re-calculated each time.

## 🔍Possible challenges:

- Probability Calculation

Finding an effective algorithm that accurately calculate the winning various card combinations and game states can be complex. 

- Input validation

Validating user input to handle errors and ensure accurate analysis results is crucial but can be challenging. Unexpected errors may occur. 

- Webpage design

## 📝References
“Texas Hold’em Poker” wikipedia: 
https://en.wikipedia.org/wiki/Texas_hold_%27em

“Texas Hold’em Poker Odd Calculator”:
https://www.888poker.com/poker/poker-odds-calculator\

“Poker Probability”: 
https://poker.fandom.com/wiki/Poker_probability

Rocket framework: 
https://rocket.rs/ 

Wasm
https://webassemblywebassembly.org/

Learning to make webpages using rust
https://www.youtube.com/watch?v=BHxmWTVFWxQ
https://www.youtube.com/watch?v=gQwA0g0NNSI&list=PLECOtlti4Psr4hXVX5GuSvLKp0-RZjz93


