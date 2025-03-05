----------------------------------------------------------------------------------
-- Author; "IvanLum"
-- 
-- Create Date: 02/25/2025 01:16:22 PM
-- Design Name: 
-- Module Name: UART - Structural
-- Project Name: 
-- Target Devices: 
-- Tool Versions: 
-- Description: 
-- 
-- Dependencies: 
-- 
-- Revision:
-- Revision 0.01 - File Created
-- Additional Comments:
-- 
----------------------------------------------------------------------------------

LIBRARY IEEE;
USE IEEE.STD_LOGIC_1164.ALL;
USE IEEE.NUMERIC_STD.ALL;
USE IEEE.STD_LOGIC_unsigned.all;
USE IEEE.STD_LOGIC_arith.all;


ENTITY UART_TB IS
    PORT (
        CLK     : IN STD_LOGIC;
        tx      : OUT STD_LOGIC;
        rx      : IN STD_LOGIC;
        led 	: OUT  STD_LOGIC_VECTOR (15 DOWNTO 0);
        JA      : IN  STD_LOGIC_VECTOR (7 DOWNTO 0);
        JC      : OUT  STD_LOGIC_VECTOR (7 DOWNTO 0)
    );
END UART_TB;

ARCHITECTURE Structural OF UART_TB IS

    COMPONENT UART IS
    GENERIC (
        BAUD_CYCLES : integer := 217 -- 460_800 baudrate
    );
    PORT (
        clk         : IN STD_LOGIC;
        data_in     : IN STD_LOGIC_VECTOR(7 DOWNTO 0);
        tx_send     : IN STD_LOGIC;
        tx_reset    : IN STD_LOGIC;
        rx          : IN STD_LOGIC;
        
        tx          : OUT STD_LOGIC;
        data_recv   : OUT STD_LOGIC_VECTOR(7 DOWNTO 0);     
        
        tx_event    : OUT STD_LOGIC;
        rx_event    : OUT STD_LOGIC
    );
    END COMPONENT;


    SIGNAL rx_event0 : STD_LOGIC := '0';
    SIGNAL rx_event1 : STD_LOGIC := '0';
    
    SIGNAL TX0_RX1 : STD_LOGIC := '0';
    SIGNAL RX0_TX1 : STD_LOGIC := '0';
    
    SIGNAL a_tx : STD_LOGIC_VECTOR(7 DOWNTO 0) := (OTHERS => '0');
    SIGNAL b_rx : STD_LOGIC_VECTOR(7 DOWNTO 0) := (OTHERS => '0');

BEGIN
    led(7 DOWNTO 0) <= b_rx;
    JC <= b_rx;
    
    led(15 DOWNTO 8) <= JA;    
    a_tx <= JA;

    UART_conn0:
    UART PORT MAP (
        clk   => CLK,
        data_in => a_tx,
        tx_send => '1',
        tx_reset => '0',
        tx => tx,
        rx => rx,
        data_recv => b_rx,
        tx_event  => OPEN,
        rx_event  => rx_event0
    );

END Structural;

