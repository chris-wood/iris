Iris Forwarder Design
=====================

Iris is a lightweight and minimalistic forwarder, akin to the CCN-Lite 
forwarder, but is intended to be implemented partially in hardware
using high-level synthesis (HLS) tools. The primary components of Iris
are: a forwarding component, Forwarding Interest Base (FIB), 
Pending Interest Table (PIT), and Content Store (CS). The forwarding
component is expected to accept CCN packets encoded according to 
[CCN wire format document]. All other components behave according
to the CCN architecture outlined in [CCN architecture document]. 



