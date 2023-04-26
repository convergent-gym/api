# Swole Control (fka GAT) API

This is a web API that clients can use to query data about a gym's business and that our hardware can use to report data about gyms.

It is written in Rust and reads/writes data to a Firebase Firestore database. This selection of database allows us to develop at a quicker pace due to its schemaless structure and allows for high performance.

Future feature buildouts will be based on the following ERD (Entity Relationship Diagram):
https://lucid.app/lucidchart/4972939f-a729-4225-97f5-c6401a20e21f/edit?viewport_loc=27%2C5%2C1922%2C1107%2C0_0&invitationId=inv_4817f8a1-085b-491a-997f-3c47beb0cea0
