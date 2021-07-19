# Command Line Interface

## Installation
Windows | Mac | Linux
--- | --- | ---
```choco install sidemash``` | ```brew install sidemash``` | ```snap install sidemash```


Once installed, you can use it under the name `sdm`.  like ```sdm -h```

## Configuration
First, log in your account and create an `AuthAccess` object to query Sidemash Cloud with API. On creation, you will have a token and a privateKey : Use them to initialize your cli client.
### 1 With cli args
It is possible to pass auth access credentials as an argument of all commands. If set the auth arguments will override env argument and config file argument described bellow.
```bash 
sdm stream-square get 1234 --auth-token 1234 --auth-secret-key 1234
```
### 2 With Env variable 
If You set the env variables ```SDM_AUTH_TOKEN``` and ```SDM_AUTH_SECRET_KEY```. Env variable has precedence over config file.

### 3 With Config file
It is possible to configure the cli with a file stored in ~/.sdm/auth
```bash 
sdm config set --auth-token 1234 --auth-secret-key 1234
```
To show your config, you can do 
```bash 
sdm config show
```

## Usage 
### Nomenclature 
The is pretty staright forward, if You have a resource that you want to Get List Update Patch or Delete, the you should  do 
 ```
sdm {resourceTypeDashedCase} {operationNameDashedCase} --{operationArgName1DashedCase} {operationArgValue1}
```

### Return code
Every command that succeed will return 0. Every command that failed will return the HTTP Error code of failure.  If you do 
```sdm stream get 1234``` and there is no `Stream` resource having id `1234`, the command will fails with the return code 404 : doing ```$?``` just after the command will yield 404. 

### Get resources
```bash
sdm stream-square get --id 1234
```

### List resources
```bash 
sdm stream-square list --where "createdTime:in:[Yesterday.14h, Yesterday.15h["
```

### Create resources
```bash
sdm stream-square create --size S --is-elastic false
```

### Update resources
```bash 
sdm stream-square update --id "1234" --new-size M
```

### Delete resources 
```bash 
sdm stream-square delete --id "1234"
```
