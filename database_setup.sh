# Create the DATABASE, if already exists will produce an error
# it's not a big deal (?) we just ignore the error
psql -h $PGHOST -U $PGUSER -c "CREATE DATABASE speculare"

# list directories in the form "./migrations/"
for dir in ./migrations/*/
do
	# remove the trailing "/"
	dir=${dir%*/}
	# apply migrations files on the database created
	psql -h $PGHOST -U $PGUSER -d speculare -f "./migrations/${dir##*/}/up.sql"
done