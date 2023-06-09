use async_openai::{
    types::{
        CreateImageEditRequest, CreateImageEditRequestArgs, CreateImageRequest,
        CreateImageRequestArgs, CreateImageVariationRequest, CreateImageVariationRequestArgs,
        ImageSize, ResponseFormat,
    },
    Client,
};
use futures::executor::block_on;

const API_USER: &str = "github.comm/vail130/openai-cli";

pub fn create(prompt: &String, n: u8, size: &str, dir: &String) -> i32 {
    let image_size = match size {
        "256" => ImageSize::S256x256,
        "512" => ImageSize::S512x512,
        "1024" => ImageSize::S1024x1024,
        s => {
            eprintln!("Invalid size. Must be 256, 512 or 1024; got {s}");
            return 1;
        }
    };
    let request_result = CreateImageRequestArgs::default()
        .prompt(prompt)
        .n(n)
        .response_format(ResponseFormat::B64Json)
        .size(image_size)
        .user(API_USER)
        .build();
    let request: CreateImageRequest = match request_result {
        Ok(req) => req,
        Err(e) => {
            eprintln!("Error creating request: {e}");
            return 1;
        }
    };
    let client = Client::new();
    let image_client = client.images();
    match block_on(image_client.create(request)) {
        Ok(image_response) => match block_on(image_response.save(dir)) {
            Ok(paths) => {
                paths.iter().for_each(|path| println!("{}", path.display()));
            }
            Err(e) => {
                eprintln!("Error writing images to disk: {e}");
                return 1;
            }
        },
        Err(e) => {
            eprintln!("Error from OpenAI: {e}");
            return 1;
        }
    };
    0
}

pub fn edit(
    image: &String,
    mask: &String,
    prompt: &String,
    n: u8,
    size: &str,
    dir: &String,
) -> i32 {
    let image_size = match size {
        "256" => ImageSize::S256x256,
        "512" => ImageSize::S512x512,
        "1024" => ImageSize::S1024x1024,
        s => {
            eprintln!("Invalid size. Must be 256, 512 or 1024; got {s}");
            return 1;
        }
    };
    let request_result = CreateImageEditRequestArgs::default()
        .image(image)
        .mask(mask)
        .prompt(prompt)
        .n(n)
        .response_format(ResponseFormat::B64Json)
        .size(image_size)
        .user(API_USER)
        .build();
    let request: CreateImageEditRequest = match request_result {
        Ok(req) => req,
        Err(e) => {
            eprintln!("Error creating request: {e}");
            return 1;
        }
    };
    let client = Client::new();
    let image_client = client.images();
    match block_on(image_client.create_edit(request)) {
        Ok(image_response) => match block_on(image_response.save(dir)) {
            Ok(paths) => {
                paths.iter().for_each(|path| println!("{}", path.display()));
            }
            Err(e) => {
                eprintln!("Error writing images to disk: {e}");
                return 1;
            }
        },
        Err(e) => {
            eprintln!("Error from OpenAI: {e}");
            return 1;
        }
    };
    0
}

pub fn variation(image: &String, n: u8, size: &str, dir: &String) -> i32 {
    let image_size = match size {
        "256" => ImageSize::S256x256,
        "512" => ImageSize::S512x512,
        "1024" => ImageSize::S1024x1024,
        s => {
            eprintln!("Invalid size. Must be 256, 512 or 1024; got {s}");
            return 1;
        }
    };
    let request_result = CreateImageVariationRequestArgs::default()
        .image(image)
        .n(n)
        .response_format(ResponseFormat::B64Json)
        .size(image_size)
        .user(API_USER)
        .build();
    let request: CreateImageVariationRequest = match request_result {
        Ok(req) => req,
        Err(e) => {
            eprintln!("Error creating request: {e}");
            return 1;
        }
    };
    let client = Client::new();
    let image_client = client.images();
    match block_on(image_client.create_variation(request)) {
        Ok(image_response) => match block_on(image_response.save(dir)) {
            Ok(paths) => {
                paths.iter().for_each(|path| println!("{}", path.display()));
            }
            Err(e) => {
                eprintln!("Error writing images to disk: {e}");
                return 1;
            }
        },
        Err(e) => {
            eprintln!("Error from OpenAI: {e}");
            return 1;
        }
    };
    0
}
