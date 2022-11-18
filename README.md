# brtc
tool collection

## tpl

convert csv to sql using br-tpl

``` sh
cat case.csv | awk 'NR>1 {print $0}' | xargs -L1 br-tpl csv --template="INSERT INTO ORDER VALUES('{{0}}', '{{1}}');" -i >> case.sql
```