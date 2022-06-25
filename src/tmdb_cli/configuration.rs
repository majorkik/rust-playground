pub const TMDB_API_URL: &str = "https://api.themoviedb.org/3";
pub const TMDB_API_KEY: &str = "6c6222ae2b7074b6260fcd8db18cb2a9";

pub enum TMDbEndpoint {
    Movie(MovieEndpoint),
}

pub enum MovieEndpoint {
    Details(u32),
    Images(u32),
    Credits(u32),
}

pub fn build_tmdb_endpoint(endpoint: TMDbEndpoint) -> String {
    let path = match endpoint {
        TMDbEndpoint::Movie(movie_endpoint) => {
            let path = match movie_endpoint {
                MovieEndpoint::Details(id) => id.to_string(),
                MovieEndpoint::Images(id) => format!("{}/images", id),
                MovieEndpoint::Credits(id) => format!("{}/credits", id),
            };

            format!("movie/{}?api_key={}", path, TMDB_API_KEY)
        }
    };

    format!("{}/{}", TMDB_API_URL, path)
}
