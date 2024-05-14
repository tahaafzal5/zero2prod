- [Preface](#preface)
  - [Ways of working](#ways-of-working)
  - [Cloud-native Applications](#cloud-native-applications)

# Preface

## Ways of working
* Trunk-based development works well to write software that is continuously deployed in a Cloud environment.
* A Gitflow approach works better for a team that sells software that is hosted and run on-premise by their customers.
* If you are working alone, you can just push straight to `main`.

## Cloud-native Applications
* We expect Cloud-native applications to:
  1. Achieve high-availability while running in fault-prone environments
  2. Allow us to continuously release new versions with zero downtime
  3. Handle dynamic workloads (e.g. request volumes)

* High availability means that our application should be able to serve requests with no downtime even if one or more of our machines suddenly starts failing.
  * This forces our application to be *distributed* - there should be multiple instances of it running on multiple machines.
* To handle dynamic workloads, we should be able to measure if our system is under load and throw more compute at the problem by spinning up new instances of the application.
  * This also requires our infrastructure to be elastic to avoid over-provisioning.
  * Running a replicated application influences our approach to data persistence - we will avoid using the local filesystem as our primary storage solution, relying instead on databases for our persistence needs.
