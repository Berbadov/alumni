// Idempotent migration for the job_postings collection.
// Run:
//   docker compose cp services/alumni-api/migrations/001_add_job_postings.js mongo:/tmp/001_add_job_postings.js
//   docker compose exec mongo mongosh --quiet /tmp/001_add_job_postings.js

const database = db.getSiblingDB(process.env.MONGO_DB || "alumni");
const collectionName = "job_postings";

if (!database.getCollectionNames().includes(collectionName)) {
  database.createCollection(collectionName);
}

const jobPostings = database.getCollection(collectionName);

jobPostings.createIndex({ author_id: 1 });
jobPostings.createIndex({ is_active: 1 });

const last = jobPostings
  .find({}, { _id: 1 })
  .sort({ _id: -1 })
  .limit(1)
  .toArray()[0];
const nextSeq = last === undefined ? NumberLong("0") : NumberLong(String(last._id));

database.counters.updateOne(
  { _id: "job_postings" },
  { $setOnInsert: { seq: nextSeq } },
  { upsert: true }
);

print("job_postings migration is complete");
