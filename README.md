# Apoptosis

Apoptosis is the work in progress migration of all IBL services to one single service that handles all the roles of Popplio, Arcadia, Persepolis and Borealis.

Internally, apoptosis uses rust to expose basic db structures etc. as userdata to the luau side which holds the actual business logic.

Apoptosis luau layer is designed to be easily modified by anyone with some knowledge of programming in Lua
