# About

Show the earthquakes that occurred at the same date from 1900 to 2022.

The data is obtained from [Catálogo Sismológico Nacional Mx](http://www2.ssn.unam.mx:8080/catalogo/).
The filters used are:

>Periodo
>
> Del 1900-01-01 al current-date

> Magnitud
>
> Desde 5.0 a 10.0

> Profundidad
>
> Todas

> Filtrar por
>
> Estado: Todos

# Run the script

Define the url of the data as enviroment variable

```bash
$ export URL_DATA="http://www2.ssn.unam.mx:8080/catalogo/json/20221003150233NFMDQ_rows.json"
```

You can use [direnv](https://direnv.net/) to manage it

## To get the url:

1. Open the `red monitor` of your browser at [Catálogo Sismológico Nacional Mx](http://www2.ssn.unam.mx:8080/catalogo/).

2. Make the search

3. Copy the `xhr` resource that has the `json` response
