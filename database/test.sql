SELECT * FROM main.create_or_replace_feature ('1b195a43-b929-4d65-8a44-420f83475bae', 'search for pokemons', 'searching for pokemons is a fun activity', '{"pokemon", "search"}');
SELECT * FROM main.create_or_replace_feature ('2b195a43-b929-4d65-8a44-420f83475bae', 'search for barbarians', 'searching for barbarians is a fun activity', '{"pokemon", "search"}');
SELECT * FROM main.create_or_replace_feature ('3b195a43-b929-4d65-8a44-420f83475bae', 'search for pirates', 'searching for pirates is a fun activity', '{"pokemon", "search"}');
SELECT * FROM main.create_or_replace_scenario ('7a92a064-add8-4be3-a764-960798fea22d', 'in the park', '{"park"}', '3b195a43-b929-4d65-8a44-420f83475bae');
SELECT * FROM main.create_or_replace_step ('4994a2ce-74a2-4888-afd5-36b44ada553b', 'given', 'I am walking in the park', '');
SELECT * FROM main.add_step_to_scenario('7a92a064-add8-4be3-a764-960798fea22d', '4994a2ce-74a2-4888-afd5-36b44ada553b');
SELECT * FROM main.delete_feature ('1b195a43-b929-4d65-8a44-420f83475bae');

INSERT INTO main.index_types VALUES ('admins');
INSERT INTO main.index_types VALUES ('streets');
INSERT INTO main.index_types VALUES ('addresses');
INSERT INTO main.index_types VALUES ('public_pois');
INSERT INTO main.index_types VALUES ('private_pois');

INSERT INTO main.data_sources VALUES ('osm');
INSERT INTO main.data_sources VALUES ('cosmogony');
INSERT INTO main.data_sources VALUES ('bano');
INSERT INTO main.data_sources VALUES ('openaddress');

INSERT INTO main.index_type_data_source VALUES('admins', 'cosmogony');
INSERT INTO main.index_type_data_source VALUES('admins', 'osm');
INSERT INTO main.index_type_data_source VALUES('streets', 'osm');
INSERT INTO main.index_type_data_source VALUES('addresses', 'bano');
INSERT INTO main.index_type_data_source VALUES('addresses', 'openaddress');
INSERT INTO main.index_type_data_source VALUES('public_pois', 'osm');
