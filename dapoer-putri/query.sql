
        CREATE or replace table  `L1_Sales.ref_consodwh_deliverytime` AS
        SELECT *
        FROM `L1_Sales.ref_consodwh_deliverytime`
        FOR SYSTEM_TIME AS OF TIMESTAMP_SUB(CURRENT_TIMESTAMP(), INTERVAL 3 HOUR);
    

        CREATE or replace table  `L1_Sales.ref_consodwh_monitoringmppkpi2022` AS
        SELECT *
        FROM `L1_Sales.ref_consodwh_monitoringmppkpi2022`
        FOR SYSTEM_TIME AS OF TIMESTAMP_SUB(CURRENT_TIMESTAMP(), INTERVAL 3 HOUR);
    

        CREATE or replace table  `L1_Sales.ref_consodwh_monitoringmppkpi2023` AS
        SELECT *
        FROM `L1_Sales.ref_consodwh_monitoringmppkpi2023`
        FOR SYSTEM_TIME AS OF TIMESTAMP_SUB(CURRENT_TIMESTAMP(), INTERVAL 3 HOUR);
    

        CREATE or replace table  `L1_Sales.ref_consodwh_src_raport` AS
        SELECT *
        FROM `L1_Sales.ref_consodwh_src_raport`
        FOR SYSTEM_TIME AS OF TIMESTAMP_SUB(CURRENT_TIMESTAMP(), INTERVAL 3 HOUR);
    

        CREATE or replace table  `L1_Sales.ref_consodwh_src_stock_subdist` AS
        SELECT *
        FROM `L1_Sales.ref_consodwh_src_stock_subdist`
        FOR SYSTEM_TIME AS OF TIMESTAMP_SUB(CURRENT_TIMESTAMP(), INTERVAL 3 HOUR);
    

        CREATE or replace table  `L1_Sales.ref_consodwh_tblm_branchregion` AS
        SELECT *
        FROM `L1_Sales.ref_consodwh_tblm_branchregion`
        FOR SYSTEM_TIME AS OF TIMESTAMP_SUB(CURRENT_TIMESTAMP(), INTERVAL 3 HOUR);
    

        CREATE or replace table  `L1_Sales.ref_consodwh_tblm_mpp_salesman_bsc` AS
        SELECT *
        FROM `L1_Sales.ref_consodwh_tblm_mpp_salesman_bsc`
        FOR SYSTEM_TIME AS OF TIMESTAMP_SUB(CURRENT_TIMESTAMP(), INTERVAL 3 HOUR);
    

        CREATE or replace table  `L1_Sales.ref_consodwh_tblm_outlet` AS
        SELECT *
        FROM `L1_Sales.ref_consodwh_tblm_outlet`
        FOR SYSTEM_TIME AS OF TIMESTAMP_SUB(CURRENT_TIMESTAMP(), INTERVAL 3 HOUR);
    

        CREATE or replace table  `L1_Sales.ref_consodwh_tblm_outlettype` AS
        SELECT *
        FROM `L1_Sales.ref_consodwh_tblm_outlettype`
        FOR SYSTEM_TIME AS OF TIMESTAMP_SUB(CURRENT_TIMESTAMP(), INTERVAL 3 HOUR);
    

        CREATE or replace table  `L1_Sales.ref_consodwh_tblm_periodreporting` AS
        SELECT *
        FROM `L1_Sales.ref_consodwh_tblm_periodreporting`
        FOR SYSTEM_TIME AS OF TIMESTAMP_SUB(CURRENT_TIMESTAMP(), INTERVAL 3 HOUR);
    

        CREATE or replace table  `L1_Sales.ref_consodwh_tblm_product` AS
        SELECT *
        FROM `L1_Sales.ref_consodwh_tblm_product`
        FOR SYSTEM_TIME AS OF TIMESTAMP_SUB(CURRENT_TIMESTAMP(), INTERVAL 3 HOUR);
    

        CREATE or replace table  `L1_Sales.ref_consodwh_tblm_store` AS
        SELECT *
        FROM `L1_Sales.ref_consodwh_tblm_store`
        FOR SYSTEM_TIME AS OF TIMESTAMP_SUB(CURRENT_TIMESTAMP(), INTERVAL 3 HOUR);
    

        CREATE or replace table  `L1_Sales.ref_consodwh_tblm_subdistribution` AS
        SELECT *
        FROM `L1_Sales.ref_consodwh_tblm_subdistribution`
        FOR SYSTEM_TIME AS OF TIMESTAMP_SUB(CURRENT_TIMESTAMP(), INTERVAL 3 HOUR);
    

        CREATE or replace table  `L1_Sales.ref_consodwh_tblt_dailyreturn` AS
        SELECT *
        FROM `L1_Sales.ref_consodwh_tblt_dailyreturn`
        FOR SYSTEM_TIME AS OF TIMESTAMP_SUB(CURRENT_TIMESTAMP(), INTERVAL 3 HOUR);
    

        CREATE or replace table  `L1_Sales.ref_consodwh_tblt_dailysales` AS
        SELECT *
        FROM `L1_Sales.ref_consodwh_tblt_dailysales`
        FOR SYSTEM_TIME AS OF TIMESTAMP_SUB(CURRENT_TIMESTAMP(), INTERVAL 3 HOUR);
    

        CREATE or replace table  `L1_Sales.ref_consodwh_tblt_dailysales_temp` AS
        SELECT *
        FROM `L1_Sales.ref_consodwh_tblt_dailysales_temp`
        FOR SYSTEM_TIME AS OF TIMESTAMP_SUB(CURRENT_TIMESTAMP(), INTERVAL 3 HOUR);
    

        CREATE or replace table  `L1_Sales.ref_consodwh_tblt_dailysales_uji` AS
        SELECT *
        FROM `L1_Sales.ref_consodwh_tblt_dailysales_uji`
        FOR SYSTEM_TIME AS OF TIMESTAMP_SUB(CURRENT_TIMESTAMP(), INTERVAL 3 HOUR);
    

        CREATE or replace table  `L1_Sales.ref_consodwh_tblt_monthlysla` AS
        SELECT *
        FROM `L1_Sales.ref_consodwh_tblt_monthlysla`
        FOR SYSTEM_TIME AS OF TIMESTAMP_SUB(CURRENT_TIMESTAMP(), INTERVAL 3 HOUR);
    

        CREATE or replace table  `L1_Sales.ref_consodwh_tblt_mpo_new_format` AS
        SELECT *
        FROM `L1_Sales.ref_consodwh_tblt_mpo_new_format`
        FOR SYSTEM_TIME AS OF TIMESTAMP_SUB(CURRENT_TIMESTAMP(), INTERVAL 3 HOUR);
    

        CREATE or replace table  `L1_Sales.ref_consodwh_tblt_target` AS
        SELECT *
        FROM `L1_Sales.ref_consodwh_tblt_target`
        FOR SYSTEM_TIME AS OF TIMESTAMP_SUB(CURRENT_TIMESTAMP(), INTERVAL 3 HOUR);
    

        CREATE or replace table  `L1_Sales.ref_consodwh_tblt_target_kam_sku` AS
        SELECT *
        FROM `L1_Sales.ref_consodwh_tblt_target_kam_sku`
        FOR SYSTEM_TIME AS OF TIMESTAMP_SUB(CURRENT_TIMESTAMP(), INTERVAL 3 HOUR);
    

        CREATE or replace table  `L1_Sales.ref_consodwh_tblt_target_kn_channel` AS
        SELECT *
        FROM `L1_Sales.ref_consodwh_tblt_target_kn_channel`
        FOR SYSTEM_TIME AS OF TIMESTAMP_SUB(CURRENT_TIMESTAMP(), INTERVAL 3 HOUR);
    

        CREATE or replace table  `L1_Sales.ref_consodwh_tblt_targetsubdist` AS
        SELECT *
        FROM `L1_Sales.ref_consodwh_tblt_targetsubdist`
        FOR SYSTEM_TIME AS OF TIMESTAMP_SUB(CURRENT_TIMESTAMP(), INTERVAL 3 HOUR);
    

        CREATE or replace table  `L1_Sales.ref_consoreporting_ms_subdist_daily` AS
        SELECT *
        FROM `L1_Sales.ref_consoreporting_ms_subdist_daily`
        FOR SYSTEM_TIME AS OF TIMESTAMP_SUB(CURRENT_TIMESTAMP(), INTERVAL 3 HOUR);
    
