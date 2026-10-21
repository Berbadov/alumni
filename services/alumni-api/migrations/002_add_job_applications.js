// Idempotent migration for the job_applications collection.
// Run after 001_add_job_postings.js:
//   docker compose cp services/alumni-api/migrations/002_add_job_applications.js mongo:/tmp/002_add_job_applications.js
//   docker compose exec mongo mongosh --quiet /tmp/002_add_job_applications.js

const database = db.getSiblingDB(process.env.MONGO_DB || "alumni");
const collectionName = "job_applications";

if (!database.getCollectionNames().includes(collectionName)) {
  database.createCollection(collectionName);
}

const applications = database.getCollection(collectionName);

applications.createIndex(
  { job_posting_id: 1, alumni_id: 1 },
  { unique: true }
);
applications.createIndex({ alumni_id: 1 });
applications.createIndex({ status: 1 });

const last = applications
  .find({}, { _id: 1 })
  .sort({ _id: -1 })
  .limit(1)
  .toArray()[0];
const nextSeq = last === undefined ? NumberLong("0") : NumberLong(String(last._id));

database.counters.updateOne(
  { _id: "job_applications" },
  { $setOnInsert: { seq: nextSeq } },
  { upsert: true }
);

print("job_applications migration is complete");
