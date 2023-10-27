# terminal-based-browser
This is (for now) a simple Rust program that can fetch and display the HTML content of a website.

To use the program, simply compile it and run it. The program will prompt you for a website URL. Once you have entered a website URL, the program will fetch the HTML content of that website and display it to the console.

After the program has finished displaying the HTML content, it will prompt you to either exit the program or paste a new URL. You can continue pasting new URLs until you hit "ctrl+s" to exit the program.

## Example

To fetch and display the HTML content of the website, you can use cargo. Navigate to the folder containing code and type "cargo run" into the terminal:

Enter the website URL: https://www.google.com/

The program would then display the HTML content of the Google website to the console.

## Requirements

The program requires the following Rust crates:

    reqwest
    select

To install these crates, you can run the following command:

cargo install reqwest
cargo install select

## Contributing

If you have any suggestions or bug reports, please feel free to open a pull request or issue on GitHub.
