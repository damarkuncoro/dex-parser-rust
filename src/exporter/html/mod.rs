use crate::dex::core::models::Apk;
use crate::exporter::core::{Exporter, ExportOptions};
use std::io::Write;

pub struct HtmlExporter;

impl Exporter for HtmlExporter {
    fn export_dex(&self, _dex: &crate::dex::core::models::Dex, _writer: &mut dyn Write, _options: &ExportOptions) -> std::io::Result<()> {
        Ok(())
    }

    fn export_apk(&self, apk: &Apk, writer: &mut dyn Write, _options: &ExportOptions) -> std::io::Result<()> {
        let json_data = serde_json::to_string(&apk.intelligence).unwrap_or_else(|_| "{}".to_string());
        let apk_name = "Android APK Analysis Report";
        let package = apk.manifest.as_ref().map(|m| m.package_name.as_str()).unwrap_or("Unknown");

        writeln!(writer, r#"<!DOCTYPE html>
<html lang="en">
<head>
    <meta charset="UTF-8">
    <meta name="viewport" content="width=device-width, initial-scale=1.0">
    <title>{title}</title>
    <script src="https://cdn.tailwindcss.com"></script>
    <script src="https://cdn.jsdelivr.net/npm/chart.js"></script>
    <script src="https://unpkg.com/vis-network/standalone/umd/vis-network.min.js"></script>
    <style>
        body {{ background-color: #f8fafc; font-family: 'Inter', sans-serif; }}
        .card {{ background: white; border-radius: 12px; box-shadow: 0 4px 6px -1px rgb(0 0 0 / 0.1); padding: 24px; }}
        .tab-active {{ border-bottom: 3px solid #2563eb; color: #2563eb; font-weight: 700; }}
        .badge {{ padding: 6px 12px; border-radius: 9999px; font-size: 0.75rem; font-weight: 700; text-transform: uppercase; }}
        .badge-critical {{ background: #fee2e2; color: #991b1b; border: 1px solid #fecaca; }}
        .badge-high {{ background: #ffedd5; color: #9a3412; border: 1px solid #fed7aa; }}
        .badge-medium {{ background: #fef9c3; color: #854d0e; border: 1px solid #fef08a; }}
        .badge-low {{ background: #f0fdf4; color: #166534; border: 1px solid #dcfce7; }}
        .badge-safe {{ background: #f1f5f9; color: #475569; border: 1px solid #e2e8f0; }}
        #network-viz {{ height: 600px; width: 100%; border: 1px solid #e2e8f0; border-radius: 12px; background: white; }}
    </style>
</head>
<body class="p-6 md:p-12">
    <div class="max-w-7xl mx-auto">
        <div class="flex flex-col md:flex-row justify-between items-start md:items-center mb-10 gap-4">
            <div>
                <h1 class="text-4xl font-black text-slate-900 tracking-tight">DEX Parser <span class="text-blue-600">Expert</span></h1>
                <p class="text-slate-500 mt-1">Package: <span class="font-mono text-blue-800 font-semibold">{package}</span></p>
            </div>
            <div id="risk-badge-container"></div>
        </div>

        <nav class="flex space-x-8 mb-8 border-b border-slate-200">
            <button onclick="showTab('summary')" id="tab-summary" class="pb-4 px-2 text-slate-500 hover:text-blue-600 transition-all tab-active">Overview</button>
            <button onclick="showTab('forensics')" id="tab-forensics" class="pb-4 px-2 text-slate-500 hover:text-blue-600 transition-all">Indicators</button>
            <button onclick="showTab('graph')" id="tab-graph" class="pb-4 px-2 text-slate-500 hover:text-blue-600 transition-all">Call Graph</button>
        </nav>

        <div id="content-summary" class="tab-content">
            <div class="grid grid-cols-1 lg:grid-cols-3 gap-8">
                <div class="lg:col-span-2 space-y-6">
                    <div class="card bg-gradient-to-br from-white to-slate-50">
                        <h2 class="text-xl font-bold text-slate-800 mb-4 flex items-center">
                            <span class="mr-2">🛡️</span> Security Intelligence
                        </h2>
                        <div id="tags-container" class="grid grid-cols-1 gap-4"></div>
                    </div>
                    <div class="card">
                        <h2 class="text-xl font-bold text-slate-800 mb-4">APK Vitals</h2>
                        <div class="grid grid-cols-2 md:grid-cols-4 gap-4 text-center" id="stats-grid"></div>
                    </div>
                </div>
                <div class="card text-center">
                    <h2 class="text-xl font-bold text-slate-800 mb-6">Anomaly Distribution</h2>
                    <div class="relative inline-block w-full">
                        <canvas id="anomalyChart"></canvas>
                    </div>
                </div>
            </div>
        </div>

        <div id="content-forensics" class="tab-content hidden">
            <div class="card">
                <div class="flex justify-between items-center mb-6">
                    <h2 class="text-xl font-bold text-slate-800">Behavioral Evidence</h2>
                    <input type="text" id="search-input" onkeyup="filterTable()" placeholder="Search indicators..."
                           class="px-4 py-2 border border-slate-200 rounded-lg text-sm focus:ring-2 focus:ring-blue-500 outline-none w-64">
                </div>
                <div class="overflow-x-auto">
                    <table class="min-w-full table-auto" id="main-table">
                        <thead class="bg-slate-50">
                            <tr>
                                <th class="px-6 py-3 text-left text-xs font-bold text-slate-500 uppercase">Category</th>
                                <th class="px-6 py-3 text-left text-xs font-bold text-slate-500 uppercase">Finding</th>
                            </tr>
                        </thead>
                        <tbody id="indicators-table" class="divide-y divide-slate-100"></tbody>
                    </table>
                </div>
            </div>
        </div>

        <div id="content-graph" class="tab-content hidden">
            <div class="card">
                <div class="flex justify-between items-center mb-4">
                    <h2 class="text-xl font-bold text-slate-800">Sensitive Call Logic</h2>
                    <p class="text-xs text-slate-400">Relations between code and suspicious APIs</p>
                </div>
                <div id="network-viz"></div>
            </div>
        </div>
    </div>

    <script id="raw-data" type="application/json">{json_data}</script>
    <script>
        const data = JSON.parse(document.getElementById('raw-data').textContent);

        function showTab(tabId) {{
            document.querySelectorAll('.tab-content').forEach(el => el.classList.add('hidden'));
            document.querySelectorAll('nav button').forEach(el => el.classList.remove('tab-active'));
            document.getElementById('content-' + tabId).classList.remove('hidden');
            document.getElementById('tab-' + tabId).classList.add('tab-active');
            if (tabId === 'graph') initGraph();
        }}

        function filterTable() {{
            const input = document.getElementById("search-input");
            const filter = input.value.toUpperCase();
            const tr = document.getElementById("indicators-table").getElementsByTagName("tr");
            for (let i = 0; i < tr.length; i++) {{
                const text = tr[i].innerText.toUpperCase();
                tr[i].style.display = text.indexOf(filter) > -1 ? "" : "none";
            }}
        }}

        const summary = data.global_security_summary;
        const riskLevel = data.analysis_tags.length > 0 ? data.analysis_tags[0].severity : "Safe";
        document.getElementById('risk-badge-container').innerHTML = `<span class="badge badge-${{riskLevel.toLowerCase()}} text-lg px-8 py-3 shadow-sm">${{riskLevel}} RISK</span>`;

        document.getElementById('stats-grid').innerHTML = `
            <div class="p-4 bg-slate-50 rounded-lg">
                <div class="text-2xl font-black text-red-600">${{summary.total_suspicious_gaps}}</div>
                <div class="text-[10px] text-slate-400 uppercase font-bold">Suspicious Gaps</div>
            </div>
            <div class="p-4 bg-slate-50 rounded-lg">
                <div class="text-2xl font-black text-amber-600">${{summary.total_sensitive_indicators}}</div>
                <div class="text-[10px] text-slate-400 uppercase font-bold">Sensitive APIs</div>
            </div>
            <div class="p-4 bg-slate-50 rounded-lg">
                <div class="text-2xl font-black text-blue-600">${{summary.total_spec_violations}}</div>
                <div class="text-[10px] text-slate-400 uppercase font-bold">Spec Violations</div>
            </div>
            <div class="p-4 bg-slate-50 rounded-lg">
                <div class="text-2xl font-black text-slate-600">${{(summary.total_dead_code / 1000).toFixed(1)}}K</div>
                <div class="text-[10px] text-slate-400 uppercase font-bold">Dead Instructions</div>
            </div>
        `;

        document.getElementById('tags-container').innerHTML = data.analysis_tags.map(tag => `
            <div class="p-5 border border-slate-200 rounded-xl bg-white flex justify-between items-start">
                <div>
                    <h3 class="font-bold text-slate-800 text-lg">${{tag.name}}</h3>
                    <p class="text-slate-500 text-sm mt-1 mb-2">${{tag.description}}</p>
                    <span class="text-xs font-mono bg-blue-50 text-blue-600 px-2 py-1 rounded">${{tag.mitre_id || "N/A"}}</span>
                </div>
                <span class="badge badge-${{tag.severity.toLowerCase()}}">${{tag.severity}}</span>
            </div>
        `).join('') || "<div class='text-slate-400 text-center py-10'>No critical threats detected.</div>";

        document.getElementById('indicators-table').innerHTML = data.behavioral_indicators.map(ind => {{
            let details = ind.content;
            if (ind.details) {{
                details += `<div class='mt-2 p-3 bg-slate-50 border-l-4 border-blue-500 rounded text-xs text-slate-600'>
                    <b>Algorithm:</b> ${{ind.details.algorithm}} | <b>Mode:</b> ${{ind.details.mode}}<br/>
                    <b>Risk:</b> <span class='text-red-600 font-bold'>${{ind.details.risk}}</span> - ${{ind.details.reason}}
                </div>`;
            }}
            return `
                <tr class="hover:bg-slate-50 transition-colors">
                    <td class="px-6 py-4 text-sm font-bold text-blue-600 whitespace-nowrap">${{ind.category}}</td>
                    <td class="px-6 py-4 text-sm text-slate-600">${{details}}</td>
                </tr>
            `;
        }}).join('');

        new Chart(document.getElementById('anomalyChart'), {{
            type: 'doughnut',
            data: {{
                labels: ['Gaps', 'APIs', 'Spec', 'Dead'],
                datasets: [{{
                    data: [summary.total_suspicious_gaps, summary.total_sensitive_indicators, summary.total_spec_violations, summary.total_dead_code],
                    backgroundColor: ['#ef4444', '#f59e0b', '#3b82f6', '#94a3b8'],
                    borderWidth: 0,
                    spacing: 5
                }}]
            }},
            options: {{ cutout: '70%', plugins: {{ legend: {{ position: 'bottom', labels: {{ usePointStyle: true, boxWidth: 6 }} }} }} }}
        }});

        function initGraph() {{
            const nodes = [];
            const edges = [];
            const seenNodes = new Set();

            Object.entries(data.cross_dex_calls).forEach(([target, callers]) => {{
                if (!seenNodes.has(target)) {{
                    nodes.push({{ id: target, label: target.split('->').pop(), group: 'api', color: '#f59e0b', shadow: true }});
                    seenNodes.add(target);
                }}
                callers.slice(0, 5).forEach(site => {{
                    const callerId = site.class_name + site.method_name;
                    if (!seenNodes.has(callerId)) {{
                        nodes.push({{ id: callerId, label: site.method_name, group: 'code', color: '#3b82f6', shadow: true }});
                        seenNodes.add(callerId);
                    }}
                    edges.push({{ from: callerId, to: target }});
                }});
            }});

            const container = document.getElementById('network-viz');
            const netData = {{ nodes: new vis.DataSet(nodes), edges: new vis.DataSet(edges) }};
            const options = {{
                nodes: {{ shape: 'dot', size: 16, font: {{ size: 12, face: 'Inter' }} }},
                edges: {{ arrows: 'to', color: '#cbd5e1', width: 1 }},
                physics: {{ enabled: true, barnesHut: {{ gravitationalConstant: -2000, centralGravity: 0.3, springLength: 95, springConstant: 0.04, damping: 0.09 }} }}
            }};
            new vis.Network(container, netData, options);
        }}
    </script>
</body>
</html>"#,
    title = apk_name,
    package = package,
    json_data = json_data)?;

        Ok(())
    }
}
