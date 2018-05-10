Simple rust app to generate a large number of randomized json records.

To compile:

`cargo build --release`

To run: 

`./data-generator -c config.json -n 1000 -o data.json -p`

## Configuration

Example configuration can be found in [config.json](config.json).

## Supported data types:

#### Text
Data type: `Text`

Configuration: `text_config`

Params:

`words` - integer; number of words in the text.

#### Date
Data type: `Date`

Configuration: `date_config`

Params:

`min_date` - date string in "%Y-%m-%d %H:%M:%S" format; minimum possible date.

`max_date` - date string in "%Y-%m-%d %H:%M:%S" format; maximum possible date.

#### Number
Data type: `Number`

Configuration: `number_config`

Params:

`min` - integer; minimum possible number.

`max` - integer; maximum possible number.

#### Name
Data type: `Name`

Params:

`None`

#### Email
Data type: `Email`

Params:

`None`

#### IPv4
Data type: `IPv4`

Params:

`None`