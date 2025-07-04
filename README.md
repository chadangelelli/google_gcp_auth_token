# `google_gcp_auth_token`

This is a very simple library, written in Rust, that will print
an auth token usable in HTTP REST requests for GCP services. It
gets the token using the `gcp_auth` crate.

# Usage

This little program simply prints out the token or error and
returns a status of `0` (success) or `1` (error).

## Prerequisites

You will want to make sure you are logged into GCP before running.
For instance, you can call the `gcloud auth application-default login`
command before using this program.

## Compiling

```
git clone https://github.com/chadangelelli/google_gcp_auth_token.git

cd google_gcp_auth_token

cargo build --release
```

## Usage from BASH

```
# After you have compiled the binary:

cd target/release

token=$(./google_gcp_auth_token)

return_code=$?
if [[ $return_code -eq 0 ]]; then
    echo "$token"
else
    echo "Error: ${token}"
fi

#= ya29.a06NzlYYwJ2VAS3Hn2uEyYihhdip4AOzOzGkmKbaheKUEWsQh...
```
