START=$(date +%s)
for x in {0..1000}; do
  # curl -s "http://localhost:8000/DTM" > /dev/null &
  curl -s "http://localhost:8000/old/DTM" > /dev/null &
  pidlist="$pidlist $!" 
done
# for x in {0..100}; do
#   curl -s "http://localhost:8000/DTM" > /dev/null &
#   pidlist="$pidlist $!" 
# done
# for x in {0..100}; do
#   curl -s "http://localhost:8000/DTM" > /dev/null &
#   pidlist="$pidlist $!" 
# done

for job in $pidlist; do 
  echo "$job"
  wait $job || let "FAIL+=1"
done  

FINISH=$(date +%s)
TIME=$(expr $FINISH - $START)

if [ "$FAIL" == "0" ]; then 
  echo "YAY!" 
else 
  echo "FAIL! ($FAIL)" 
fi

echo "Time was $TIME"
