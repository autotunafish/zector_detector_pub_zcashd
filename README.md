# zector_detector_pub_zcashd
A local explorer that uses a public, free, zcashd node for querying information. 
* Use getblock.io to create a free account, only email required.
* Create a 'shared' Zcash node.
* Use the Node IP Address Endpoint and contained API Key in the Explorer.
* 
* For Linux, Requires Rust and inotifytools
*
* git clone <https://github.com/autotunafish/zector_detector_pub_zcashd.git>
* cd zector_detector_pub_zcashd
* cargo run
* 
* Apply executable permissions to
* * /action_notify
  * /action_notify_b
  * /chain_notify
  * /chaininfo1.txt
  * /action1.txt
  * /action4.txt
* 
* You will be prompted to input your Node IP Address and API Key.
* 
