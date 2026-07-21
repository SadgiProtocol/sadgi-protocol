import { NextResponse } from 'next/server';

export async function GET() {
  const timestamp = Date.now();
  
  const activeJobs = Math.floor(Math.random() * 100);
  const connectedProvers = Math.floor(Math.random() * 20);
  
  const metrics = 
# HELP sadgi_active_jobs Number of currently active ZK jobs in the marketplace
# TYPE sadgi_active_jobs gauge
sadgi_active_jobs  

# HELP sadgi_connected_provers Number of SP1 provers connected to the network
# TYPE sadgi_connected_provers gauge
sadgi_connected_provers  

# HELP sadgi_dashboard_uptime_seconds Dashboard uptime
# TYPE sadgi_dashboard_uptime_seconds counter
sadgi_dashboard_uptime_seconds  
\;

  return new NextResponse(metrics, {
    status: 200,
    headers: {
      'Content-Type': 'text/plain; version=0.0.4',
    },
  });
}
