# KG Access API

The demo Knowledge Graph is accessed via the STARDOG API - however all work is done using **stored queries**.

Parameters are passed in the form ${param}={value} - URL encoded.

The sequence of calls to access data for the demo is:

```
curl --request GET -u username:password --header "Accept: application/sparql-results+json" "https://sd-59a2b7ca.stardog.cloud:5820/prov-chains/query?query=getSpatialEntity&%24entity=%3Chttp%3A%2F%2Fexample.com%2Fdata%2Fc%3E"
curl --request GET -u username:password --header "Accept: application/sparql-results+json" "https://sd-59a2b7ca.stardog.cloud:5820/prov-chains/query?query=getObject&%24object=%3Chttp%3A%2F%2Fexample.com%2Fdata%2Fc%3E"
curl --request GET -u username:password --header "Accept: application/sparql-results+json" "https://sd-59a2b7ca.stardog.cloud:5820/prov-chains/query?query=getActivity4Entity&%24entity=%3Chttp%3A%2F%2Fexample.com%2Fdata%2Fc%3E"
curl --request GET -u username:password --header "Accept: application/sparql-results+json" "https://sd-59a2b7ca.stardog.cloud:5820/prov-chains/query?query=getObject&%24object=%3Chttp%3A%2F%2Fexample.com%2Factivities%2Fadd1%3E" 
curl --request GET -u username:password --header "Accept: application/sparql-results+json" "https://sd-59a2b7ca.stardog.cloud:5820/prov-chains/query?query=getObject&%24object=%3Chttp%3A%2F%2Fexample.com%2Fdata%2Fa%3E" 
curl --request GET -u username:password --header "Accept: application/sparql-results+json" "https://sd-59a2b7ca.stardog.cloud:5820/prov-chains/query?query=getObject&%24object=%3Chttp%3A%2F%2Fexample.com%2Fdata%2Fb%3E" 
``` 

Data is returned in a standard SPARQL results format using JSON
