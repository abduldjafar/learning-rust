def generate(table):
    query = f"""
        CREATE or replace table  `L1_Sales.{table}` AS
        SELECT *
        FROM `L1_Sales.{table}`
        FOR SYSTEM_TIME AS OF TIMESTAMP_SUB(CURRENT_TIMESTAMP(), INTERVAL 3 HOUR);
    """

    return query

tables = [{
  "table_id": "ref_consodwh_deliverytime"
}, {
  "table_id": "ref_consodwh_monitoringmppkpi2022"
}, {
  "table_id": "ref_consodwh_monitoringmppkpi2023"
}, {
  "table_id": "ref_consodwh_src_raport"
}, {
  "table_id": "ref_consodwh_src_stock_subdist"
}, {
  "table_id": "ref_consodwh_tblm_branchregion"
}, {
  "table_id": "ref_consodwh_tblm_mpp_salesman_bsc"
}, {
  "table_id": "ref_consodwh_tblm_outlet"
}, {
  "table_id": "ref_consodwh_tblm_outlettype"
}, {
  "table_id": "ref_consodwh_tblm_periodreporting"
}, {
  "table_id": "ref_consodwh_tblm_product"
}, {
  "table_id": "ref_consodwh_tblm_store"
}, {
  "table_id": "ref_consodwh_tblm_subdistribution"
}, {
  "table_id": "ref_consodwh_tblt_dailyreturn"
}, {
  "table_id": "ref_consodwh_tblt_dailysales"
}, {
  "table_id": "ref_consodwh_tblt_dailysales_temp"
}, {
  "table_id": "ref_consodwh_tblt_dailysales_uji"
}, {
  "table_id": "ref_consodwh_tblt_monthlysla"
}, {
  "table_id": "ref_consodwh_tblt_mpo_new_format"
}, {
  "table_id": "ref_consodwh_tblt_target"
}, {
  "table_id": "ref_consodwh_tblt_target_kam_sku"
}, {
  "table_id": "ref_consodwh_tblt_target_kn_channel"
}, {
  "table_id": "ref_consodwh_tblt_targetsubdist"
}, {
  "table_id": "ref_consoreporting_ms_subdist_daily"
}]

for data in tables:
    print(generate(data["table_id"]))