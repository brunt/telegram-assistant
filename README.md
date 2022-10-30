# My personal Telegram bot
This is the codebase for my [Telegram](https://telegram.org/) bot. It's a chatbot that I talk to via the Telegram app on my phone and the code in this repository runs on my [Raspberry Pi 3B+](https://www.raspberrypi.org/products/raspberry-pi-3-model-b-plus/).
I use it to:
* lookup schedules of St. Louis Metro trains
* check the weather forecast via [OpenWeatherMap](https://openweathermap.org/api)
* track my spending
* Get news articles from [NewsAPI](https://newsapi.org/)

More functionality can be implemented as needed.

### How it works
The bot does these things by polling Telegram for messages that I send, then sending HTTP requests to other services, some of which I have running on my local network.
Once a response is received from one of those services, it is parsed into a message and sent as a response message to me on Telegram.

### Architecture in a nutshell
![chatbot diagram](media/diagram.png?raw)

### You can run it too!
This code is open source. If you want to run my telegram bot, simply supply your own Telegram bot token as well as any other environment variables necessary.
You are more than welcome to fork my code and change it to accommodate your own needs.

### How to run
* Git clone this repo
* Have [Rust](https://www.rust-lang.org/) installed
* Add necessary environment variables
* Run `cargo run` in your terminal while in this project directory

### Additional resources
* [Overview of what Telegram is](https://telegram.org/faq)
* [Telegram.org documentation about bots](https://core.telegram.org/bots)
* [Teloxide getting started guide](https://github.com/teloxide/teloxide#getting-started)