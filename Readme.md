# Setup

### Installation

Make sure you have the latests Rust version installed,
Clone the repo and run 
```
cargo run
```
in your console

### Requests
In order for movie memo to make requests to TMDB you will need to get an API key which you can get following [these instructions](https://developers.themoviedb.org/3/getting-started/introduction). Once you have your api key
you will need to add them to a .env file in the root folder of the project:

```
# .env
API_KEY_V3 = {{YOUR TMDB API KEY GOES HERE}}
```

In order to use test requests in the `requests` folder, you will need to install the [REST Client](https://marketplace.visualstudio.com/items?itemName=humao.rest-client) extension for VSCode. Once installed 
you'll need to add your api key with the following configuration to your `.vscode/settings.json` file:
```
// .vscode/settings.json
{
    ...
    "rest-client.environmentVariables": {
        "$shared": {
            "API_KEY_V3": "{{YOUR TMDB API KEY GOES HERE}}"
        }
    }
    ...
}
```