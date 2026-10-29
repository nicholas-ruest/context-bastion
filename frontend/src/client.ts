export type Role='user'|'assistant'|'tool'|'system';
export interface Event{id:string;session_id:string;sequence:number;role:Role;source:string;content:string;authenticated:boolean;digest:string}
export interface Claim{id:string;text:string;event_ids:string[];roles:Role[];trusted:boolean}
export interface Summary{text:string;claims:Claim[];compacted_count:number;laundering_markers:number}
export interface Decision{allowed:boolean;reason:string;policy_id:string}
export interface Receipt{id:string;action_id:string;effect_count:number;digest:string}
export interface Session{id:string;name:string;events:Event[];summary?:Summary;decisions:Decision[];receipts:Receipt[];analogues:{claim_id:string;distance:number}[]}
const API='http://127.0.0.1:8787';
async function call<T>(path:string,init?:RequestInit):Promise<T>{const res=await fetch(API+path,{...init,headers:{'content-type':'application/json','authorization':'Bearer operator-token','x-correlation-id':crypto.randomUUID(),...(init?.headers??{})}});if(!res.ok){const body=await res.json().catch(()=>({message:res.statusText}));throw new Error(body.message??res.statusText)}return res.json() as Promise<T>}
export const createSession=(name:string)=>call<Session>('/api/v1/sessions',{method:'POST',body:JSON.stringify({name})});
export const addEvent=(id:string,role:Role,source:string,content:string,authenticated=false)=>call<Session>(`/api/v1/sessions/${id}/events`,{method:'POST',body:JSON.stringify({role,source,content,authenticated})});
export const compact=(id:string)=>call<Session>(`/api/v1/sessions/${id}/compact`,{method:'POST'});
export const execute=(id:string,verb:string,target:string,risk:string,approval_event_id?:string)=>call<Session>(`/api/v1/sessions/${id}/actions`,{method:'POST',body:JSON.stringify({verb,target,risk,approval_event_id})});
export const getSession=(id:string)=>call<Session>(`/api/v1/sessions/${id}`);
export async function restart():Promise<void>{await call('/api/v1/admin/shutdown',{method:'POST'}).catch(()=>undefined);for(let i=0;i<80;i+=1){await new Promise(r=>setTimeout(r,125));if(await fetch(API+'/healthz').then(r=>r.ok).catch(()=>false))return}throw new Error('backend did not restart')}
