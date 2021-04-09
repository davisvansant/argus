#### [ Argus ]

[![Build Status](https://cloud.drone.io/api/badges/davisvansant/argus/status.svg)](https://cloud.drone.io/davisvansant/argus)

An in `in-memory` "secrets" store, built on `cryptography` primitives

(note that some of the items built here would not be considered best practice, and is purely for learning purposes)

`Rand` - create random

 - salts
 - pins
 - account numbers
 - AES keys
 - nonces

`AES` - encrypted data

`UUID` - creates unique IDs for secrets

`SHA-2` - hashing algorithm for encoding plaintext passwords and salts

`ed25519` - sign encrypted secrets

`x25519` - creates a key pair for system and account to communicate
