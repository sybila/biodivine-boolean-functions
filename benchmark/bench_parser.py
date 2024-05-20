# Taken from BBM model 122
# https://github.com/sybila/biodivine-boolean-models/blob/main/models/%5Bid-122%5D__%5Bvar-74%5D__%5Bin-94%5D__%5BNSP14%5D/model.bnet

EXPRESSIONS = [
	"((v_Aminoimidazole_ribotide_simple_molecule & v_CO2_simple_molecule) & v_PAICS)",
"(((v_1__5_Phospho_D_ribosyl__5_amino_4_imidazolecarboxylate_simple_molecule & v_L_Aspartate_simple_molecule) & v_ATP_simple_molecule) & v_PAICS)",
"(v_1__5__Phosphoribosyl__5_amino_4__N_succinocarboxamide__imidazole_simple_molecule & v_ADSL)",
"((v_1__5__Phosphoribosyl__5_amino_4_imidazolecarboxamide_simple_molecule & v_10_Formyltetrahydrofolate_simple_molecule) & v_ATIC)",
"((((v_5_phosphoribosyl_N_formylglycinamide_simple_molecule & v_L_Glutamine_simple_molecule) & v_ATP_simple_molecule) & v_H2O_simple_molecule_cell) & v_PFAS)",
"((v_Deoxyguanosine_simple_molecule & v_Pi_simple_molecule) & v_PNP)",
"((((v_PRPS1 | v_PRPS2) | v_PRPS1L1) & v_D_Ribose_5P_simple_molecule) & v_ATP_simple_molecule)",
"(((v_5_phospho__alpha__D_ribose_1_diphosphate_simple_molecule & v_H2O_simple_molecule_cell) & v_L_Glutamine_simple_molecule) & v_PPAT)",
"(((v_5_phospho_beta_D_ribosylamine_simple_molecule & v_Glycine_simple_molecule) & v_ATP_simple_molecule) & v_GART)",
"((v_5_phospho_beta_D_ribosylglycinamide_simple_molecule & v_10_Formyltetrahydrofolate_simple_molecule) & v_GART)",
"((v_NAD_simple_molecule & v_H2O_simple_molecule_cell) & v_CD38)",
"(((((((((((((((((((((v__alpha__D_Galactose_simple_molecule & v_ATP_simple_molecule) & v_GALK1) | ((v_NAD_simple_molecule & v_ATP_simple_molecule) & v_NADK)) | ((((v_Deamino_NAD_simple_molecule & v_L_Glutamine_simple_molecule) & v_ATP_simple_molecule) & v_H2O_simple_molecule_cell) & v_NADSYN1)) | ((v_N_Ribosyl_nicotinamide_simple_molecule & v_ATP_simple_molecule) & v_NRK1)) | ((((v_Nicotinate_simple_molecule & v_5_phospho__alpha__D_ribose_1_diphosphate_simple_molecule) & v_H2O_simple_molecule_cell) & v_ATP_simple_molecule) & v_NAPRT1)) | (((v_5_phospho_beta_D_ribosylamine_simple_molecule & v_Glycine_simple_molecule) & v_ATP_simple_molecule) & v_GART)) | ((((v_5_phosphoribosyl_N_formylglycinamide_simple_molecule & v_L_Glutamine_simple_molecule) & v_ATP_simple_molecule) & v_H2O_simple_molecule_cell) & v_PFAS)) | ((v_2__Formamido__N1__5__phosphoribosyl_acetamidine_simple_molecule & v_ATP_simple_molecule) & v_GART)) | (((v_1__5_Phospho_D_ribosyl__5_amino_4_imidazolecarboxylate_simple_molecule & v_L_Aspartate_simple_molecule) & v_ATP_simple_molecule) & v_PAICS)) | ((v_GMP_simple_molecule & v_ATP_simple_molecule) & v_GUK1)) | ((((((v_NME3 | v_csa2_Nucleoside_diphosphate_kinase_complex_cell) | v_NME5) | v_NME6) | v_NME7) & v_GDP_simple_molecule) & v_ATP_simple_molecule)) | ((((((v_NME3 | v_csa2_Nucleoside_diphosphate_kinase_complex_cell) | v_NME7) | v_NME6) | v_NME5) & v_dGDP_simple_molecule) & v_ATP_simple_molecule)) | ((v_dGMP_simple_molecule & v_ATP_simple_molecule) & v_GUK1)) | ((v_Deoxyguanosine_simple_molecule & v_ATP_simple_molecule) & v_DCK)) | ((v_Adenosine_simple_molecule & v_ATP_simple_molecule) & v_ADK)) | (((((v_AK7 | v_AK1) | v_AK8) | v_AK5) & v_AMP_simple_molecule) & v_ATP_simple_molecule)) | ((v_Deoxyadenosine_simple_molecule & v_ATP_simple_molecule) & v_DCK)) | ((v_dAMP_simple_molecule & v_ATP_simple_molecule) & v_AK5)) | ((((((v_csa5_Nucleoside_diphosphate_kinase_complex_cell | v_NME5) | v_NME3) | v_NME6) | v_NME7) & v_dADP_simple_molecule) & v_ATP_simple_molecule)) | (((((v_AK7 | v_AK1) | v_AK8) | v_AK5) & v_AMP_simple_molecule) & v_ATP_simple_molecule))",
"(((((((v_ENPP1 | v_ENPP3) & v_NAD_simple_molecule) & v_H2O_simple_molecule_cell) | ((((v_PRPS1 | v_PRPS2) | v_PRPS1L1) & v_D_Ribose_5P_simple_molecule) & v_ATP_simple_molecule)) | ((((v_XMP_simple_molecule & v_H2O_simple_molecule_cell) & v_ATP_simple_molecule) & v_L_Glutamine_simple_molecule) & v_GMPS)) | ((v_Adenosine_simple_molecule & v_ATP_simple_molecule) & v_ADK)) | ((v_Adenine_simple_molecule & v_5_phospho__alpha__D_ribose_1_diphosphate_simple_molecule) & v_APRT))",
"((v_Adenosine_simple_molecule & v_Pi_simple_molecule) & v_PNP)",
"((v_AMP_simple_molecule & v_H2O_simple_molecule_cell) & v_NT5E)",
"((v_2__Formamido__N1__5__phosphoribosyl_acetamidine_simple_molecule & v_ATP_simple_molecule) & v_GART)",
"((((((((v_AMDP2 | v_AMPD1) | v_AMPD3) & v_AMP_simple_molecule) & v_H_simple_molecule) & v_H2O_simple_molecule_cell) | (((v_Guanine_simple_molecule & v_H2O_simple_molecule_cell) & v_H_simple_molecule) & v_GDA)) | (((v_Adenosine_simple_molecule & v_H2O_simple_molecule_cell) & v_H_simple_molecule) & v_ADA)) | (((v_Deoxyadenosine_simple_molecule & v_H2O_simple_molecule_cell) & v_H_simple_molecule) & v_ADA))",
"((((v_Quinolinate_simple_molecule & v_5_phospho__alpha__D_ribose_1_diphosphate_simple_molecule) & v_H_simple_molecule) & v_H_simple_molecule) & v_QPRT)",
"(((((((v_GLB1 | v_LCT) & v_Lactose_simple_molecule) & v_H2O_simple_molecule_cell) | ((v_Galacitol_simple_molecule & v_NADP_simple_molecule) & v_AKR1B1)) | ((((v_GLA | v_GLA_Nsp14_complex) & v_Melibiose_simple_molecule) & v_H2O_simple_molecule_cell) & !v_Migalastat_drug)) | ((((v_GLA | v_GLA_Nsp14_complex) & v_Raffinose_simple_molecule) & v_H2O_simple_molecule_cell) & !v_Migalastat_drug)) | ((((v_GLA | v_GLA_Nsp14_complex) & v_Stachyose_simple_molecule) & v_H2O_simple_molecule_cell) & !v_Migalastat_drug))",
"(v__alpha_D_Ribose_1P_simple_molecule & v_PGM2)",
"(((((v_NMNAT1 | v_NMNAT2) | v_NMNAT3) & v_Nicotinate_D_ribonucleotide_simple_molecule) & v_H_simple_molecule) & v_ATP_simple_molecule)",
"(((v_Hypoxanthine_simple_molecule & v_2_deoxy__alpha__D_ribose_1_phosphate_simple_molecule) & v_PNP) | (((v_Deoxyadenosine_simple_molecule & v_H2O_simple_molecule_cell) & v_H_simple_molecule) & v_ADA))",
"(v_1__5__Phosphoribosyl__5_amino_4__N_succinocarboxamide__imidazole_simple_molecule & v_ADSL)",
"(((v_GMP_simple_molecule & v_ATP_simple_molecule) & v_GUK1) | (((v_ENTPD2 | v_NTPCR) & v_GTP_simple_molecule) & v_H2O_simple_molecule_cell))",
"(v_GLA & v_Nsp14_cell)",
"((((((((v_XMP_simple_molecule & v_H2O_simple_molecule_cell) & v_ATP_simple_molecule) & v_L_Glutamine_simple_molecule) & v_GMPS) | ((v_Guanine_simple_molecule & v_5_phospho__alpha__D_ribose_1_diphosphate_simple_molecule) & v_HPRT1)) | ((v_GTP_simple_molecule & v_H2O_simple_molecule_cell) & v_ENTPD2)) | ((((((v_ENTPD2 | v_ENTPD4) | v_ENTPD5) | v_ENTPD6) | v_CANT1) & v_GDP_simple_molecule) & v_H2O_simple_molecule_cell)) | ((((v_GMPR | v_GMPR2) & v_IMP_simple_molecule) & v_Ammonium_simple_molecule) & v_NADP_simple_molecule))",
"((((((v_NME3 | v_csa2_Nucleoside_diphosphate_kinase_complex_cell) | v_NME5) | v_NME6) | v_NME7) & v_GDP_simple_molecule) & v_ATP_simple_molecule)",
"v_IMPDH2",
"(((v_Deoxyguanosine_simple_molecule & v_Pi_simple_molecule) & v_PNP) | ((v_Guanosine_simple_molecule & v_Pi_simple_molecule) & v_PNP))",
"((v_GMP_simple_molecule & v_H2O_simple_molecule_cell) & v_NT5E)",
"(((v_1__5__Phosphoribosyl__5_formamido_4_imidazolecarboxamide_simple_molecule & v_ATIC) | ((v_GDP_simple_molecule & v_Thioredoxin_simple_molecule) & v_csa3_ribonucleoside_reductase_complex_cell)) | ((v_ADP_simple_molecule & v_Thioredoxin_simple_molecule) & v_csa4_ribonucleoside_reductase_complex_cell))",
"(((((((((((((((((((((((((((v_Galacitol_simple_molecule & v_NADP_simple_molecule) & v_AKR1B1) | ((v_NAD_simple_molecule & v_ATP_simple_molecule) & v_NADK)) | ((((v_Deamino_NAD_simple_molecule & v_L_Glutamine_simple_molecule) & v_ATP_simple_molecule) & v_H2O_simple_molecule_cell) & v_NADSYN1)) | ((v_N_Ribosyl_nicotinamide_simple_molecule & v_ATP_simple_molecule) & v_NRK1)) | ((v_NAD_simple_molecule & v_H2O_simple_molecule_cell) & v_CD38)) | (((v_5_phospho_beta_D_ribosylamine_simple_molecule & v_Glycine_simple_molecule) & v_ATP_simple_molecule) & v_GART)) | ((v_5_phospho_beta_D_ribosylglycinamide_simple_molecule & v_10_Formyltetrahydrofolate_simple_molecule) & v_GART)) | ((((v_5_phosphoribosyl_N_formylglycinamide_simple_molecule & v_L_Glutamine_simple_molecule) & v_ATP_simple_molecule) & v_H2O_simple_molecule_cell) & v_PFAS)) | ((v_2__Formamido__N1__5__phosphoribosyl_acetamidine_simple_molecule & v_ATP_simple_molecule) & v_GART)) | ((v_Aminoimidazole_ribotide_simple_molecule & v_CO2_simple_molecule) & v_PAICS)) | ((v_Aminoimidazole_ribotide_simple_molecule & v_CO2_simple_molecule) & v_PAICS)) | (((v_1__5_Phospho_D_ribosyl__5_amino_4_imidazolecarboxylate_simple_molecule & v_L_Aspartate_simple_molecule) & v_ATP_simple_molecule) & v_PAICS)) | ((((((((v_IMPDH1 | v_IMPDH2) | v_IMPDH2_Nsp14_complex) & v_IMP_simple_molecule) & v_NAD_simple_molecule) & v_H2O_simple_molecule_cell) & !v_Mycophenolic_acid_drug) & !v_Merimepodib_drug) & !v_Ribavirin_drug)) | ((((v_XMP_simple_molecule & v_H2O_simple_molecule_cell) & v_ATP_simple_molecule) & v_L_Glutamine_simple_molecule) & v_GMPS)) | ((((v_XMP_simple_molecule & v_H2O_simple_molecule_cell) & v_ATP_simple_molecule) & v_L_Glutamine_simple_molecule) & v_GMPS)) | ((v_Deoxyguanosine_simple_molecule & v_ATP_simple_molecule) & v_DCK)) | ((((v_ITPA | v_ENPP1) | v_ENPP3) & v_dGTP_simple_molecule) & v_H2O_simple_molecule_cell)) | ((v_GTP_simple_molecule & v_H2O_simple_molecule_cell) & v_ENTPD2)) | ((v_GTP_simple_molecule & v_H2O_simple_molecule_cell) & v_ENTPD2)) | (((v_ENTPD2 | v_NTPCR) & v_GTP_simple_molecule) & v_H2O_simple_molecule_cell)) | ((((((v_ENTPD2 | v_ENTPD4) | v_ENTPD5) | v_ENTPD6) | v_CANT1) & v_GDP_simple_molecule) & v_H2O_simple_molecule_cell)) | ((((v_GMPR | v_GMPR2) & v_IMP_simple_molecule) & v_Ammonium_simple_molecule) & v_NADP_simple_molecule)) | ((((v_GMPR | v_GMPR2) & v_IMP_simple_molecule) & v_Ammonium_simple_molecule) & v_NADP_simple_molecule)) | (((v_Hypoxanthine_simple_molecule & v_NAD_simple_molecule) & v_H2O_simple_molecule_cell) & v_XDH)) | ((v_Adenosine_simple_molecule & v_ATP_simple_molecule) & v_ADK)) | ((v_Deoxyadenosine_simple_molecule & v_ATP_simple_molecule) & v_DCK))",
"((((v_SIRT5 | v_SIRT5_Nsp14_complex) & v_NAD_simple_molecule) & v_Histone_N6_acetyl_L_lysine_simple_molecule) & v_H2O_simple_molecule_mitochondrion)",
"((v_Inosine_simple_molecule & v_Pi_simple_molecule) & v_PNP)",
"(v_IMPDH2 & v_Nsp14_cell)",
"(((v_1__5__Phosphoribosyl__5_formamido_4_imidazolecarboxamide_simple_molecule & v_ATIC) | (((((v_AMDP2 | v_AMPD1) | v_AMPD3) & v_AMP_simple_molecule) & v_H_simple_molecule) & v_H2O_simple_molecule_cell)) | ((v_Hypoxanthine_simple_molecule & v_5_phospho__alpha__D_ribose_1_diphosphate_simple_molecule) & v_HPRT1))",
"(((v_IMP_simple_molecule & v_H2O_simple_molecule_cell) & v_NT5E) | (((v_Adenosine_simple_molecule & v_H2O_simple_molecule_cell) & v_H_simple_molecule) & v_ADA))",
"(((((((v_Deamino_NAD_simple_molecule & v_L_Glutamine_simple_molecule) & v_ATP_simple_molecule) & v_H2O_simple_molecule_cell) & v_NADSYN1) | (((v_5_phospho__alpha__D_ribose_1_diphosphate_simple_molecule & v_H2O_simple_molecule_cell) & v_L_Glutamine_simple_molecule) & v_PPAT)) | ((((v_5_phosphoribosyl_N_formylglycinamide_simple_molecule & v_L_Glutamine_simple_molecule) & v_ATP_simple_molecule) & v_H2O_simple_molecule_cell) & v_PFAS)) | ((((v_XMP_simple_molecule & v_H2O_simple_molecule_cell) & v_ATP_simple_molecule) & v_L_Glutamine_simple_molecule) & v_GMPS))",
"((v_UDP__alpha__D_Galactose_simple_molecule & v__alpha__D_Glucose_simple_molecule) & v_lactose_synthetase_complex)",
"((((v_NAD_simple_molecule & v_NADPH_simple_molecule) & v_NNT) | ((((((((v_IMPDH1 | v_IMPDH2) | v_IMPDH2_Nsp14_complex) & v_IMP_simple_molecule) & v_NAD_simple_molecule) & v_H2O_simple_molecule_cell) & !v_Mycophenolic_acid_drug) & !v_Merimepodib_drug) & !v_Ribavirin_drug)) | (((v_Hypoxanthine_simple_molecule & v_NAD_simple_molecule) & v_H2O_simple_molecule_cell) & v_XDH))",
"(((v_Galacitol_simple_molecule & v_NADP_simple_molecule) & v_AKR1B1) | ((((v_GMPR | v_GMPR2) & v_IMP_simple_molecule) & v_Ammonium_simple_molecule) & v_NADP_simple_molecule))",
"(((v_NAD_simple_molecule & v_ATP_simple_molecule) & v_NADK) | ((v_NAD_simple_molecule & v_NADPH_simple_molecule) & v_NNT))",
"(((((v_Deamino_NAD_simple_molecule & v_L_Glutamine_simple_molecule) & v_ATP_simple_molecule) & v_H2O_simple_molecule_cell) & v_NADSYN1) | (((((v_NMNAT2 | v_NMNAT1) | v_NMNAT3) & v_Nicotinamide_D_ribonucleotide_simple_molecule) & v_ATP_simple_molecule) & v_H_simple_molecule))",
"((v_Nicotinamide_D_ribonucleotide_simple_molecule & v_H2O_simple_molecule_cell) & v_NT5E)",
"(((((v_ENPP1 | v_ENPP3) & v_NAD_simple_molecule) & v_H2O_simple_molecule_cell) | ((v_N_Ribosyl_nicotinamide_simple_molecule & v_ATP_simple_molecule) & v_NRK1)) | ((v_Nicotinamide_simple_molecule & v_5_phospho__alpha__D_ribose_1_diphosphate_simple_molecule) & v_NAMPT))",
"((((((v_Nicotinate_simple_molecule & v_NADP_simple_molecule) & v_CD38) | ((v_N_Ribosyl_nicotinamide_simple_molecule & v_Pi_simple_molecule) & v_PNP)) | ((((v_SIRT5 | v_SIRT5_Nsp14_complex) & v_NAD_simple_molecule) & v_Histone_N6_acetyl_L_lysine_simple_molecule) & v_H2O_simple_molecule_mitochondrion)) | ((v_NAD_simple_molecule & v_H2O_simple_molecule_cell) & v_CD38)) | v_Nicotinate_simple_molecule)",
"(((((v_Quinolinate_simple_molecule & v_5_phospho__alpha__D_ribose_1_diphosphate_simple_molecule) & v_H_simple_molecule) & v_H_simple_molecule) & v_QPRT) | ((((v_Nicotinate_simple_molecule & v_5_phospho__alpha__D_ribose_1_diphosphate_simple_molecule) & v_H2O_simple_molecule_cell) & v_ATP_simple_molecule) & v_NAPRT1))",
"((((v_SIRT5 | v_SIRT5_Nsp14_complex) & v_NAD_simple_molecule) & v_Histone_N6_acetyl_L_lysine_simple_molecule) & v_H2O_simple_molecule_mitochondrion)",
"(((((((((((((((v_NMNAT2 | v_NMNAT1) | v_NMNAT3) & v_Nicotinamide_D_ribonucleotide_simple_molecule) & v_ATP_simple_molecule) & v_H_simple_molecule) | ((v_Nicotinamide_simple_molecule & v_5_phospho__alpha__D_ribose_1_diphosphate_simple_molecule) & v_NAMPT)) | (((((v_NMNAT1 | v_NMNAT2) | v_NMNAT3) & v_Nicotinate_D_ribonucleotide_simple_molecule) & v_H_simple_molecule) & v_ATP_simple_molecule)) | ((((v_Quinolinate_simple_molecule & v_5_phospho__alpha__D_ribose_1_diphosphate_simple_molecule) & v_H_simple_molecule) & v_H_simple_molecule) & v_QPRT)) | ((((v_Nicotinate_simple_molecule & v_5_phospho__alpha__D_ribose_1_diphosphate_simple_molecule) & v_H2O_simple_molecule_cell) & v_ATP_simple_molecule) & v_NAPRT1)) | (((v_5_phospho__alpha__D_ribose_1_diphosphate_simple_molecule & v_H2O_simple_molecule_cell) & v_L_Glutamine_simple_molecule) & v_PPAT)) | ((((v_XMP_simple_molecule & v_H2O_simple_molecule_cell) & v_ATP_simple_molecule) & v_L_Glutamine_simple_molecule) & v_GMPS)) | ((((v_ITPA | v_ENPP1) | v_ENPP3) & v_dGTP_simple_molecule) & v_H2O_simple_molecule_cell)) | ((v_Guanine_simple_molecule & v_5_phospho__alpha__D_ribose_1_diphosphate_simple_molecule) & v_HPRT1)) | ((v_Hypoxanthine_simple_molecule & v_5_phospho__alpha__D_ribose_1_diphosphate_simple_molecule) & v_HPRT1)) | ((v_Adenine_simple_molecule & v_5_phospho__alpha__D_ribose_1_diphosphate_simple_molecule) & v_APRT))",
"((((((((((((((((v_Nicotinamide_D_ribonucleotide_simple_molecule & v_H2O_simple_molecule_cell) & v_NT5E) | ((((v_Nicotinate_simple_molecule & v_5_phospho__alpha__D_ribose_1_diphosphate_simple_molecule) & v_H2O_simple_molecule_cell) & v_ATP_simple_molecule) & v_NAPRT1)) | (((v_5_phospho_beta_D_ribosylamine_simple_molecule & v_Glycine_simple_molecule) & v_ATP_simple_molecule) & v_GART)) | ((((v_5_phosphoribosyl_N_formylglycinamide_simple_molecule & v_L_Glutamine_simple_molecule) & v_ATP_simple_molecule) & v_H2O_simple_molecule_cell) & v_PFAS)) | ((v_2__Formamido__N1__5__phosphoribosyl_acetamidine_simple_molecule & v_ATP_simple_molecule) & v_GART)) | (((v_1__5_Phospho_D_ribosyl__5_amino_4_imidazolecarboxylate_simple_molecule & v_L_Aspartate_simple_molecule) & v_ATP_simple_molecule) & v_PAICS)) | ((v_GMP_simple_molecule & v_H2O_simple_molecule_cell) & v_NT5E)) | ((v_GTP_simple_molecule & v_H2O_simple_molecule_cell) & v_ENTPD2)) | ((v_GTP_simple_molecule & v_H2O_simple_molecule_cell) & v_ENTPD2)) | (((v_ENTPD2 | v_NTPCR) & v_GTP_simple_molecule) & v_H2O_simple_molecule_cell)) | ((((((v_ENTPD2 | v_ENTPD4) | v_ENTPD5) | v_ENTPD6) | v_CANT1) & v_GDP_simple_molecule) & v_H2O_simple_molecule_cell)) | ((v_XMP_simple_molecule & v_H2O_simple_molecule_cell) & v_NT5E)) | ((v_IMP_simple_molecule & v_H2O_simple_molecule_cell) & v_NT5E)) | ((v_AMP_simple_molecule & v_H2O_simple_molecule_cell) & v_NT5E)) | ((v_Hypoxanthine_simple_molecule & v_2_deoxy__alpha__D_ribose_1_phosphate_simple_molecule) & v_PNP))",
"((((v_GLA | v_GLA_Nsp14_complex) & v_Stachyose_simple_molecule) & v_H2O_simple_molecule_cell) & !v_Migalastat_drug)",
"(v_SIRT5 & v_Nsp14_mitochondrion)",
"((((v_GLA | v_GLA_Nsp14_complex) & v_Raffinose_simple_molecule) & v_H2O_simple_molecule_cell) & !v_Migalastat_drug)",
"(((v_5_phospho_beta_D_ribosylglycinamide_simple_molecule & v_10_Formyltetrahydrofolate_simple_molecule) & v_GART) | ((v_1__5__Phosphoribosyl__5_amino_4_imidazolecarboxamide_simple_molecule & v_10_Formyltetrahydrofolate_simple_molecule) & v_ATIC))",
"(((v_GDP_simple_molecule & v_Thioredoxin_simple_molecule) & v_csa3_ribonucleoside_reductase_complex_cell) | ((v_ADP_simple_molecule & v_Thioredoxin_simple_molecule) & v_csa4_ribonucleoside_reductase_complex_cell))",
"(((v__alpha__D_Galactose_1P_simple_molecule & v_UDP__alpha__D_Glucose_simple_molecule) & v_GALT) | (v_UDP__alpha__D_Glucose_simple_molecule & v_GALE))",
"((v_UDP__alpha__D_Galactose_simple_molecule & v__alpha__D_Glucose_simple_molecule) & v_lactose_synthetase_complex)",
"((v_UDP__alpha__D_Glucose_simple_molecule & v_PPi_simple_molecule) & v_UGP2)",
"v_SIRT5",
"((((((((v_IMPDH1 | v_IMPDH2) | v_IMPDH2_Nsp14_complex) & v_IMP_simple_molecule) & v_NAD_simple_molecule) & v_H2O_simple_molecule_cell) & !v_Mycophenolic_acid_drug) & !v_Merimepodib_drug) & !v_Ribavirin_drug)",
"((((v_Xanthosine_simple_molecule & v_Pi_simple_molecule) & v_PNP) | (((v_Hypoxanthine_simple_molecule & v_NAD_simple_molecule) & v_H2O_simple_molecule_cell) & v_XDH)) | (((v_Guanine_simple_molecule & v_H2O_simple_molecule_cell) & v_H_simple_molecule) & v_GDA))",
"((v_XMP_simple_molecule & v_H2O_simple_molecule_cell) & v_NT5E)",
"((v__alpha__D_Galactose_simple_molecule & v_ATP_simple_molecule) & v_GALK1)",
"(v_D_Galactose_simple_molecule & v_GALM)",
"(((v__alpha__D_Galactose_1P_simple_molecule & v_UDP__alpha__D_Glucose_simple_molecule) & v_GALT) | ((v_UDP__alpha__D_Glucose_simple_molecule & v_PPi_simple_molecule) & v_UGP2))",
"((((v_GLB1 | v_LCT) & v_Lactose_simple_molecule) & v_H2O_simple_molecule_cell) | ((((v_GLA | v_GLA_Nsp14_complex) & v_Melibiose_simple_molecule) & v_H2O_simple_molecule_cell) & !v_Migalastat_drug))",
"((((((v_N_Ribosyl_nicotinamide_simple_molecule & v_Pi_simple_molecule) & v_PNP) | ((v_Guanosine_simple_molecule & v_Pi_simple_molecule) & v_PNP)) | ((v_Xanthosine_simple_molecule & v_Pi_simple_molecule) & v_PNP)) | ((v_Inosine_simple_molecule & v_Pi_simple_molecule) & v_PNP)) | ((v_Adenosine_simple_molecule & v_Pi_simple_molecule) & v_PNP))",
"(((v_dAMP_simple_molecule & v_ATP_simple_molecule) & v_AK5) | ((v_ADP_simple_molecule & v_Thioredoxin_simple_molecule) & v_csa4_ribonucleoside_reductase_complex_cell))",
"((v_Deoxyadenosine_simple_molecule & v_ATP_simple_molecule) & v_DCK)",
"((((((v_csa5_Nucleoside_diphosphate_kinase_complex_cell | v_NME5) | v_NME3) | v_NME6) | v_NME7) & v_dADP_simple_molecule) & v_ATP_simple_molecule)",
"(((v_GDP_simple_molecule & v_Thioredoxin_simple_molecule) & v_csa3_ribonucleoside_reductase_complex_cell) | ((v_dGMP_simple_molecule & v_ATP_simple_molecule) & v_GUK1))",
"(((v_Deoxyguanosine_simple_molecule & v_ATP_simple_molecule) & v_DCK) | ((((v_ITPA | v_ENPP1) | v_ENPP3) & v_dGTP_simple_molecule) & v_H2O_simple_molecule_cell))",
"((((((v_NME3 | v_csa2_Nucleoside_diphosphate_kinase_complex_cell) | v_NME7) | v_NME6) | v_NME5) & v_dGDP_simple_molecule) & v_ATP_simple_molecule)",
"((v_Nicotinate_simple_molecule & v_NADP_simple_molecule) & v_CD38)"
]
EXPRESSIONS = sorted(EXPRESSIONS, key = lambda x: len(x))

import biodivine_boolean_functions as bbf
from pyeda.inter import *
import time

def parse_bbf(e):
	return bbf.Expression(e)

def parse_pyeda(e):
	return expr(e)

REPETITIONS = 10
SKIP_SIZE = 2000

print("size\tBBF(parse)\tBBF(CNF)\tPyEDA(parse)\tPyEDA(CNF)")

for e in EXPRESSIONS:

	if len(e) >= SKIP_SIZE:
		print("Skip CNF")

	total_bbf = 0	
	bbf_cnf = 0
	for _ in range(REPETITIONS):
		start = time.perf_counter_ns()
		parsed = parse_bbf(e)		
		total_bbf += time.perf_counter_ns() - start				
		start = time.perf_counter_ns()
		if len(e) < SKIP_SIZE:
			parsed.to_cnf()		
		bbf_cnf += time.perf_counter_ns() - start

	e = e.replace("!", "~")

	total_pyeda = 0
	pyeda_cnf = 0
	for _ in range(REPETITIONS):
		start = time.perf_counter_ns()
		parsed = parse_pyeda(e)		
		total_pyeda += time.perf_counter_ns() - start	
		start = time.perf_counter_ns()
		if len(e) < SKIP_SIZE:
			parsed.to_cnf()		
		pyeda_cnf += time.perf_counter_ns() - start

	print(f"{len(e)}\t{int(total_bbf / REPETITIONS)}\t{int(bbf_cnf / REPETITIONS)}\t{int(total_pyeda / REPETITIONS)}\t{int(pyeda_cnf / REPETITIONS)}")


