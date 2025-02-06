// Copyright 2025 zTgx
//
// Licensed under the Apache License, Version 2.0 (the "License");
// you may not use this file except in compliance with the License.
// You may obtain a copy of the License at
//
// http://www.apache.org/licenses/LICENSE-2.0
//
// Unless required by applicable law or agreed to in writing, software
// distributed under the License is distributed on an "AS IS" BASIS,
// WITHOUT WARRANTIES OR CONDITIONS OF ANY KIND, either express or implied.
// See the License for the specific language governing permissions and
// limitations under the License.

use solagent_plugin_goplus::get_token_security_info;

#[tokio::main]
async fn main() {
    let chain_id = "42161";
    let mint = "0xEa51801b8F5B88543DdaD3D1727400c15b209D8f";

    let check = get_token_security_info(chain_id, mint).await.unwrap();
    println!("Token check: {:?}", check);
}
