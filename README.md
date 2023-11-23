# zector_detector_pub_zcashd
A local explorer that uses a public, free, zcashd node for querying information. 
* Use getblock.io to create a free account, only email required.
* Create a 'shared' Zcash node.
* Use the Node IP Address Endpoint and contained API Key in the Explorer.
* 
* For Linux, Requires Rust and inotifytools
*
* git clone https://github.com/autotunafish/zector_detector_pub_zcashd.git
* cd zector_detector_pub_zcashd
* cargo run
* 
* Apply executable permissions to
* * /x-action_notify
  * /x-action_notify_b
  * /x-chain_notify
  * /x-chaininfo1.txt
  * /x-action1.txt
  * /x-action4.txt
* 
* You will be prompted to input your Node IP Address and API Key.
* 
* Hit Enter To Refresh Info, Input TX Hash Or Block Hash/Height
* Input 'R' For Raw Json
* <img src="https://github.com/autotunafish/zector_detector_pub_zcashd/blob/main/images/zector1.png"  width="665" height="750" />
